//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 977/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk977<F: Float>(t10648: F, t10649: F, t10650: F, t2768: F, t11582: F, t1654: F, t1102: F, t11572: F, t3314: F, t10609: F, t498: F, t97: F, t10943: F, t11603: F, t2333: F, t2847: F) -> (F, F, F, F, F, F) {
    let t39251 = t10648 * t10649 * t10650 * t2768;
    let t39255 = t10648 * t10649 * t11582 * t1654;
    let t39260 = t1102 * t3314 * t11572;
    let t39263 = t97 * t10609 * t498;
    let t39290 = t10943 * t11603;
    let t39299 = t2333 * t2847;
    (t39251, t39255, t39260, t39263, t39290, t39299)
}
