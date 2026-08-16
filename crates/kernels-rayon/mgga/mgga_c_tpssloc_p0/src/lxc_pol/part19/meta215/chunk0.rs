//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 912/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk912(t10647: f64, t291: f64, t2784: f64, t892: f64, t914: f64, t2787: f64, t2837: f64, t2841: f64, t888: f64, t2845: f64, t10521: f64, t10528: f64, t10607: f64, t10622: f64, t10625: f64, t10627: f64, t10635: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10649 = 0.621814e-1_f64 * t10647 * t291;
    let t10650 = t2784 * t892;
    let t10652 = 3.0_f64 * t10650 * t914;
    let t10654 = 3.0_f64 * t2787 * t2837;
    let t10655 = t888 * t2841;
    let t10657 = 0.48245938496077605201e2_f64 * t10655 * t2845;
    let t10658 = -t10521 + t10528 - t10607 + t10622 - t10625 - t10627 - t10635 - t10649 + t10652 + t10654 + t10657;
    (t10649, t10650, t10652, t10654, t10655, t10657, t10658)
}
