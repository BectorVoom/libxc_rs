//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 685/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk685(t5269: f64, t5270: f64, t5267: f64, t234: f64, t1743: f64, t704: f64, t740: f64, t717: f64, t749: f64, t225: f64, t1836: f64, t703: f64, t716: f64) -> (f64, f64, f64, f64, f64) {
    let t5271 = t5269 * t5270;
    let t5272 = t5267 * t5271;
    let t5274 = 0.91082604192152556044e5_f64 * t234 * t5272;
    let t5275 = t704 * t1743;
    let t5276 = t5275 * t740;
    let t5278 = 0.35089341735807877242e1_f64 * t234 * t5276;
    let t5279 = t717 * t1743;
    let t5280 = t5279 * t749;
    let t5282 = 0.51947577317044391277e2_f64 * t234 * t5280;
    let t5285 = t225 * t5270;
    let t5286 = t1836 * t5285;
    let t5288 = 0.14035736694323150897e2_f64 * t234 * t5286;
    let t5290 = 1.0_f64 / t716 / t703;
    (t5274, t5278, t5282, t5288, t5290)
}
