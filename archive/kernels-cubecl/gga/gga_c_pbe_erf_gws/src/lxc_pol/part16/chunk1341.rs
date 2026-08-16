//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1341/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1341<F: Float>(t54301: F, t54305: F, t51383: F, t51388: F, t51396: F, t51401: F, t54295: F, t54297: F, t54299: F, t54303: F, t54307: F, t54310: F) -> F {
    let t55580 = F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t54301;
    let t55582 = F::cast_from(119.0_f64) / F::cast_from(1728.0_f64) * t54305;
    let t55586 = -F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t51383 - F::cast_from(119.0_f64) / F::cast_from(864.0_f64) * t51388 - F::cast_from(119.0_f64) / F::cast_from(432.0_f64) * t51396 + t54295 / F::cast_from(24.0_f64) - t54297 / F::cast_from(12.0_f64) + t54299 / F::cast_from(24.0_f64) + t55580 + F::cast_from(5.0_f64) / F::cast_from(96.0_f64) * t54303 - t55582 - t54307 / F::cast_from(24.0_f64) - F::cast_from(35.0_f64) / F::cast_from(288.0_f64) * t51401 + t54310 / F::cast_from(96.0_f64);
    t55586
}
