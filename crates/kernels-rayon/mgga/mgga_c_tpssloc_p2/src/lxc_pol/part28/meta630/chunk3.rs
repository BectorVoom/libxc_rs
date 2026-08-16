//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1976/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1976(t87165: f64, t87177: f64, t26653: f64, t814: f64, t87520: f64, t1509: f64, t7084: f64, t87522: f64, t13171: f64, t1510: f64, t24256: f64, t2617: f64, t26598: f64, t26662: f64, t4166: f64, t4291: f64, t7101: f64, t812: f64, t81615: f64, t81617: f64, t829: f64, t84945: f64, t87171: f64, t87174: f64, t87517: f64, t87527: f64) -> (f64, f64) {
    let t92530 = 0.3289868133696452873e-1_f64 * t87165;
    let t92543 = 0.16449340668482264365e-1_f64 * t87177;
    let t92546 = t814 * t26653;
    let t92551 = 0.3289868133696452873e-1_f64 * t87520;
    let t92552 = t7084 * t1509;
    let t92556 = 0.15352717957250113407e0_f64 * t87522;
    let t92558 = t92530 + 0.3289868133696452873e-1_f64 * t81615 - t812 * t84945 * t1510 - 0.38381794893125283518e-1_f64 * t81617 + 2.0_f64 * t4166 * t24256 - 2.0_f64 * t2617 * t26662 - t812 * t7101 * t13171 + 0.6579736267392905746e-1_f64 * t87171 - 0.3289868133696452873e-1_f64 * t87174 + t92543 - 2.0_f64 * t2617 * t26598 - 2.0_f64 * t812 * t92546 * t829 + 0.3289868133696452873e-1_f64 * t87517 - t92551 - 2.0_f64 * t4291 * t92552 * t829 + t92556 - 0.3289868133696452873e-1_f64 * t87527;
    (t92552, t92558)
}
