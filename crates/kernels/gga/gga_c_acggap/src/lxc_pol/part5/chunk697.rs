//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 697/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk697<F: Float>(t3396: F, t3403: F, t3571: F, t3574: F, t3576: F, t3622: F, t3624: F, t3634: F, t3636: F, t3638: F, t3649: F, t3653: F, t3658: F, t418: F, t4901: F, t4906: F, t4910: F, t4912: F, t4918: F, t4921: F, t4926: F, t4928: F, t4932: F) -> F {
    let t4945 = F::cast_from(0.80031500487063509014e-2_f64) * t4901 - t4906 + t4910 + F::cast_from(0.68598428988911579156e-2_f64) * t3396 * t4912 + t4918 - F::cast_from(0.42874018118069736972e-2_f64) * t3403 * t4921 - F::cast_from(0.42874018118069736972e-3_f64) * t4926 - F::cast_from(0.20007875121765877254e-2_f64) * t4928 + F::cast_from(0.34299214494455789578e-2_f64) * t418 * t4932 - F::cast_from(35.0_f64) / F::cast_from(108.0_f64) * t3571 - F::cast_from(35.0_f64) / F::cast_from(216.0_f64) * t3574 + F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t3576 - F::cast_from(7.0_f64) / F::cast_from(48.0_f64) * t3622 - F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t3624 + F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t3634 + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t3636 + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t3638 + t3649 + F::cast_from(0.17149607247227894789e-2_f64) * t3653 - F::cast_from(0.17149607247227894789e-2_f64) * t3658;
    t4945
}
