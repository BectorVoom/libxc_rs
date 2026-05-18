//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1262/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1262<F: Float>(t54128: F, t54135: F, t54152: F, t54166: F, t54198: F, t54236: F, t54238: F, t54257: F, t54259: F, t54267: F, t54271: F, t54283: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t55487 = F::new(7.0) / F::new(288.0) * t54128;
    let t55491 = F::new(7.0) / F::new(72.0) * t54135;
    let t55500 = F::new(7.0) / F::new(72.0) * t54152;
    let t55508 = F::new(7.0) / F::new(72.0) * t54166;
    let t55524 = F::new(7.0) / F::new(288.0) * t54198;
    let t55547 = F::new(7.0) / F::new(72.0) * t54236;
    let t55548 = F::new(7.0) / F::new(144.0) * t54238;
    let t55556 = F::new(7.0) / F::new(72.0) * t54257;
    let t55557 = F::new(7.0) / F::new(36.0) * t54259;
    let t55562 = F::new(7.0) / F::new(36.0) * t54267;
    let t55564 = F::new(7.0) / F::new(72.0) * t54271;
    let t55569 = F::new(7.0) / F::new(288.0) * t54283;
    (t55487, t55491, t55500, t55508, t55524, t55547, t55548, t55556, t55557, t55562, t55564, t55569)
}
