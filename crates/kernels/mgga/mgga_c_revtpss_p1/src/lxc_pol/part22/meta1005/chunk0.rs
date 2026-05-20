//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3436/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3436<F: Float>(t15573: F, t4719: F, t11524: F, t19133: F, t981: F, t15526: F, t19134: F, t3022: F, t15266: F, t52894: F, t63597: F, t19021: F, t3011: F) -> (F, F, F, F, F, F) {
    let t64493 = F::cast_from(0.2077903092681775651e3_f64) * t4719 * t15573;
    let t64496 = F::cast_from(0.10389515463408878255e3_f64) * t981 * t19133 * t11524;
    let t64498 = F::cast_from(0.69263436422725855034e2_f64) * t4719 * t15526;
    let t64500 = F::cast_from(0.20779030926817756511e3_f64) * t3022 * t19134;
    let t64503 = F::cast_from(0.41016075432865626631e4_f64) * t52894 * t15266 * t63597;
    let t64504 = t3011 * t19021;
    (t64493, t64496, t64498, t64500, t64503, t64504)
}
