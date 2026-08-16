//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1283/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1283(t3447: f64, t4904: f64, t64779: f64, t15402: f64, t21749: f64, t22398: f64, t225: f64, t1243: f64, t72361: f64, t22334: f64, t22337: f64, t22328: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t73535 = t3447 * t64779 * t4904;
    let t73541 = t3447 * t15402 * t21749;
    let t73613 = t22398 * t225;
    let t73630 = t72361 * t1243;
    let t73856 = t22334 * t225;
    let t73891 = t22337 * t225;
    let t73900 = t22328 * t225;
    (t73535, t73541, t73613, t73630, t73856, t73891, t73900)
}
