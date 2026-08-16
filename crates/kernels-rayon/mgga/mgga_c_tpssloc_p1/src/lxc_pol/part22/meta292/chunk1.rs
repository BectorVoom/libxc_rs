//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1451/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1451(t13602: f64, t2815: f64, t4370: f64, t2798: f64, t10595: f64, t1547: f64, t10599: f64, t1553: f64, t2403: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13603 = 2.0_f64 / 9.0_f64 * t13602;
    let t13623 = t2815 * t4370;
    let t13629 = t2798 * t4370;
    let t13634 = t10595 * t1547;
    let t13637 = t10599 * t1547;
    let t13642 = t2403 * t1553;
    (t13603, t13623, t13629, t13634, t13637, t13642)
}
