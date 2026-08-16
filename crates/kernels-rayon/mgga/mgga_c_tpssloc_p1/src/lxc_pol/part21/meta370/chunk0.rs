//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1815/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1815(t13615: f64, t901: f64, t2815: f64, t4370: f64, t896: f64, t2807: f64, t4378: f64, t2798: f64, t4362: f64, t10595: f64, t1547: f64, t2799: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13616 = t901 * t13615;
    let t13623 = t2815 * t4370;
    let t13624 = t13623 * t896;
    let t13626 = t4378 * t2807;
    let t13629 = t2798 * t4370;
    let t13630 = t13629 * t896;
    let t13632 = t4362 * t2807;
    let t13634 = t10595 * t1547;
    let t13635 = t13634 * t2799;
    (t13616, t13624, t13626, t13630, t13632, t13634, t13635)
}
