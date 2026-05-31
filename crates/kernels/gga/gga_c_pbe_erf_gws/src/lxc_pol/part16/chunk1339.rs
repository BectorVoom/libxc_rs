//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1339/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1339<F: Float>(t54236: F, t54238: F, t54257: F, t54259: F, t51341: F, t51358: F, t54241: F, t54246: F, t54248: F, t54251: F, t54255: F, t54261: F) -> F {
    let t55547 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t54236;
    let t55548 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t54238;
    let t55556 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t54257;
    let t55557 = F::cast_from(7.0_f64) / F::cast_from(36.0_f64) * t54259;
    let t55559 = t55547 - t55548 - F::cast_from(7.0_f64) / F::cast_from(36.0_f64) * t51341 + t54241 / F::cast_from(24.0_f64) + t54246 / F::cast_from(12.0_f64) + t54248 / F::cast_from(96.0_f64) - F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t51358 - t54251 / F::cast_from(8.0_f64) - t54255 / F::cast_from(24.0_f64) + t55556 - t55557 - t54261 / F::cast_from(384.0_f64);
    t55559
}
