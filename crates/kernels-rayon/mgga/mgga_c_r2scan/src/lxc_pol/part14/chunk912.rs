//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 912/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk912(t322: f64, t1339: f64, t352: f64, t1338: f64, t2441: f64, t1035: f64, t6755: f64, t8397: f64, t1348: f64, t6767: f64, t1018: f64, t1307: f64, t2405: f64, t2437: f64, t2438: f64, t2445: f64, t330: f64, t6751: f64, t837: f64, t8420: f64, t8425: f64, t8454: f64, t8479: f64, t855: f64) -> (f64, f64, f64) {
    let t323 = t322 <= 0.0_f64;
    let t331 = t322 <= 0.25e1_f64;
    let t332 = 0.25e1_f64 < t322;
    let t8481 = t352 * t1339;
    let t8484 = t1338 * t2441;
    let t8487 = t6755 * t1035;
    let t8492 = piecewise3(t332, t8397, 0.0_f64);
    let t8496 = t1348 * t2441;
    let t8501 = t6767 * t1035;
    let t8505 = piecewise5(t323, t1018 * t1307 * t330 + 2.0_f64 * t2405 * t837 * t330 + t8420 * t330 + t8425 * t330, t331, t8454 + t8479, -0.63e1_f64 * t2445 * t8481 - 0.42e1_f64 * t8484 * t2438 - 0.945e1_f64 * t8487 * t8481 - 0.21e1_f64 * t2437 * t6751 - 0.105e1_f64 * t855 * t8492 * t352 - 0.315e1_f64 * t8496 * t2438 - 0.1575e1_f64 * t2445 * t6751 - 0.23625e1_f64 * t8501 * t8481);
    (t8481, t8492, t8505)
}
