//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 976/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk976(t322: f64, t1338: f64, t3416: f64, t1096: f64, t6755: f64, t11059: f64, t1348: f64, t6767: f64, t1079: f64, t11082: f64, t11087: f64, t11117: f64, t11141: f64, t1307: f64, t2438: f64, t330: f64, t3381: f64, t3413: f64, t3420: f64, t352: f64, t6751: f64, t837: f64, t8481: f64, t855: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t323 = t322 <= 0.0_f64;
    let t331 = t322 <= 0.25e1_f64;
    let t332 = 0.25e1_f64 < t322;
    let t11145 = t1338 * t3416;
    let t11148 = t6755 * t1096;
    let t11153 = piecewise3(t332, t11059, 0.0_f64);
    let t11157 = t1348 * t3416;
    let t11162 = t6767 * t1096;
    let t11166 = piecewise5(t323, t1079 * t1307 * t330 + 2.0_f64 * t3381 * t837 * t330 + t11082 * t330 + t11087 * t330, t331, t11117 + t11141, -0.63e1_f64 * t3420 * t8481 - 0.42e1_f64 * t11145 * t2438 - 0.945e1_f64 * t11148 * t8481 - 0.21e1_f64 * t3413 * t6751 - 0.105e1_f64 * t855 * t11153 * t352 - 0.315e1_f64 * t11157 * t2438 - 0.1575e1_f64 * t3420 * t6751 - 0.23625e1_f64 * t11162 * t8481);
    (t11145, t11148, t11153, t11157, t11162, t11166)
}
