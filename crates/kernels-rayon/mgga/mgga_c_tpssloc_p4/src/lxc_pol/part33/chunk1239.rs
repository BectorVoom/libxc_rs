//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1239/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1239(t1014: f64, t82514: f64, t3032: f64, t360: f64, t1009: f64, t343: f64, t25490: f64, t225: f64, t82390: f64, t3158: f64, t6796: f64, t23600: f64, t995: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t82637 = t82514 * t1014;
    let t82638 = t3032 * t360;
    let t82654 = t343 * t1009;
    let t82655 = t82654 * t25490;
    let t82676 = t82390 * t225;
    let t82716 = t6796 * t3158;
    let t82736 = t23600 * t995;
    (t82637, t82638, t82655, t82676, t82716, t82736)
}
