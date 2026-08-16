//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2054/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2054(t1014: f64, t82654: f64, t23479: f64, t25637: f64, t6722: f64, t1409: f64, t344: f64, t1009: f64, t6740: f64, t23473: f64, t3082: f64, t7586: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t88431 = t82654 * t1014;
    let t88440 = 0.16149102437656156342e-2_f64 * t6722 * t25637 * t23479;
    let t88449 = t1409 * t344;
    let t88451 = t6740 * t88449 * t1009;
    let t88453 = 0.20186378047070195428e-3_f64 * t88451 * t23473;
    let t88479 = t7586 * t3082;
    (t88431, t88440, t88449, t88451, t88453, t88479)
}
