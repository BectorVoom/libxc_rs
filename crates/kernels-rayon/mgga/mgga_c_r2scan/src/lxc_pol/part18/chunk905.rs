//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 905/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk905(t322: f64, t2441: f64, t352: f64, t2983: f64, t6755: f64, t1338: f64, t2987: f64, t9675: f64, t1348: f64, t6767: f64, t1019: f64, t2405: f64, t2437: f64, t2438: f64, t2445: f64, t2951: f64, t2953: f64, t2991: f64, t330: f64, t837: f64, t855: f64, t9698: f64, t9731: f64, t9756: f64) -> (f64, f64, f64) {
    let t323 = t322 <= 0.0_f64;
    let t331 = t322 <= 0.25e1_f64;
    let t332 = 0.25e1_f64 < t322;
    let t9760 = t352 * t2441;
    let t9763 = t6755 * t2983;
    let t9766 = t1338 * t2987;
    let t9769 = piecewise3(t332, t9675, 0.0_f64);
    let t9773 = t1348 * t2987;
    let t9778 = t6767 * t2983;
    let t9782 = piecewise5(t323, t2951 * t837 * t330 + t2953 * t837 * t330 + 2.0_f64 * t1019 * t2405 + t9698 * t330, t331, t9731 + t9756, -0.63e1_f64 * t2991 * t2438 - 0.42e1_f64 * t2437 * t9760 - 0.945e1_f64 * t9763 * t2438 - 0.21e1_f64 * t9766 * t2438 - 0.105e1_f64 * t855 * t9769 * t352 - 0.1575e1_f64 * t9773 * t2438 - 0.315e1_f64 * t2445 * t9760 - 0.23625e1_f64 * t9778 * t2438);
    (t9760, t9769, t9782)
}
