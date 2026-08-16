//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1104/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1104(t11584: f64, t37369: f64, t10648: f64, t10649: f64, t10650: f64, t2768: f64, t11582: f64, t1654: f64, t1102: f64, t11572: f64, t3314: f64, t10609: f64, t498: f64, t97: f64) -> (f64, f64, f64, f64, f64) {
    let t39247 = t37369 * t11584;
    let t39251 = t10648 * t10649 * t10650 * t2768;
    let t39252 = 0.72042316457491791906e-3_f64 * t39251;
    let t39255 = t10648 * t10649 * t11582 * t1654;
    let t39256 = 0.72042316457491791906e-3_f64 * t39255;
    let t39260 = t1102 * t3314 * t11572;
    let t39261 = 0.81300399444200075504e-3_f64 * t39260;
    let t39263 = t97 * t10609 * t498;
    (t39247, t39252, t39256, t39261, t39263)
}
