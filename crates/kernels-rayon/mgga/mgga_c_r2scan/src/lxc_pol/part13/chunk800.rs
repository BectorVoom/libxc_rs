//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 800/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk800(t1048: f64, t7040: f64, t795: f64, t2266: f64, t2267: f64, t2526: f64, t2271: f64, t2810: f64, t2813: f64, t2452: f64, t410: f64, t372: f64, t4845: f64, t7025: f64, t7028: f64, t7031: f64, t7033: f64, t7036: f64, t7039: f64) -> (f64, f64) {
    let t7042 = t1048 * t7040 * t795;
    let t7043 = 2.0_f64 * t7042;
    let t7045 = t2266 * t2267 * t2526;
    let t7046 = 6.0_f64 * t7045;
    let t7048 = 0.4726e1_f64 * t2271 * t2810;
    let t7050 = 0.4726e1_f64 * t2271 * t2813;
    let t7051 = t410 * t2452;
    let t7052 = 8.0_f64 * t7051;
    let t7053 = t372 * t7028 + t4845 - t7025 - t7031 - t7033 + t7036 - t7039 + t7043 - t7046 - t7048 - t7050 + t7052;
    (t7052, t7053)
}
