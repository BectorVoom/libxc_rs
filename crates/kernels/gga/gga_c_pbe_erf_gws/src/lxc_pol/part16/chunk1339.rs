//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1339/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1339<F: Float>(t54236: F, t54238: F, t54257: F, t54259: F, t51341: F, t51358: F, t54241: F, t54246: F, t54248: F, t54251: F, t54255: F, t54261: F) -> F {
    let t55547 = F::new(7.0) / F::new(72.0) * t54236;
    let t55548 = F::new(7.0) / F::new(144.0) * t54238;
    let t55556 = F::new(7.0) / F::new(72.0) * t54257;
    let t55557 = F::new(7.0) / F::new(36.0) * t54259;
    let t55559 = t55547 - t55548 - F::new(7.0) / F::new(36.0) * t51341 + t54241 / F::new(24.0) + t54246 / F::new(12.0) + t54248 / F::new(96.0) - F::new(7.0) / F::new(144.0) * t51358 - t54251 / F::new(8.0) - t54255 / F::new(24.0) + t55556 - t55557 - t54261 / F::new(384.0);
    t55559
}
