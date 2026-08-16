//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2350/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2350(t68077: f64, t68102: f64, t68124: f64, t68141: f64, t225: f64, t21008: f64, t9573: f64, t13228: f64, t1495: f64, t1510: f64, t16662: f64, t16836: f64, t16851: f64, t16928: f64, t210: f64, t237: f64, t249: f64, t2571: f64, t2643: f64, t41130: f64, t41139: f64, t41363: f64, t4178: f64, t46692: f64, t47039: f64, t47080: f64, t47094: f64, t47231: f64, t47270: f64, t58569: f64, t59100: f64) -> (f64, f64, f64) {
    let t68143 = t68077 + t68102 + t68124 + t68141;
    let t68144 = t68143 * t225;
    let t68148 = t9573 * t21008;
    let t68150 = 3.0_f64 / 16.0_f64 * t2571 * t210 * t1495 * t16662 + t47080 - 595.0_f64 / 10368.0_f64 * t41130 + t41139 - t47094 + 595.0_f64 / 10368.0_f64 * t41363 - t47231 + 7.0_f64 / 4.0_f64 * t59100 + 3.0_f64 / 512.0_f64 * t4178 * t46692 * t13228 * t58569 - t16836 * t16928 / 64.0_f64 + 15.0_f64 / 128.0_f64 * t2643 * t47039 * t1510 * t16851 + t68144 * t237 * t249 / 3072.0_f64 - 7.0_f64 / 16.0_f64 * t68148 - t47270;
    (t68143, t68144, t68150)
}
