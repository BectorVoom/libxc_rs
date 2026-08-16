//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 921/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk921(t33973: f64, t858: f64, t1527: f64, t8740: f64, t2718: f64, t1528: f64, t2054: f64, t26700: f64, t26713: f64, t31964: f64, t32023: f64, t32027: f64, t33449: f64, t33459: f64, t4147: f64, t4268: f64, t855: f64, t8734: f64, t8741: f64) -> (f64, f64, f64) {
    let t33974 = t858 * t33973;
    let t33981 = t8740 * t1527;
    let t33982 = t2718 * t33981;
    let t33989 = -2.0_f64 * t26713 * t2054 + 2.0_f64 * t4268 * t8734 - t855 * t33974 + 2.0_f64 * t4147 * t8734 + t32023 + 0.6579736267392905746e-1_f64 * t33449 + 0.6579736267392905746e-1_f64 * t33459 - t31964 * t1528 + 2.0_f64 * t855 * t33982 - 2.0_f64 * t26700 * t2054 - t4147 * t8741 - t4268 * t8741 + t32027;
    (t33974, t33982, t33989)
}
