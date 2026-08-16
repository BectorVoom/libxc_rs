//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 717/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk717(t6657: f64, t829: f64, t235: f64, t6624: f64, t1909: f64, t226: f64, t6636: f64, t6641: f64, t6645: f64, t6650: f64, t6654: f64, t808: f64, t812: f64) -> (f64, f64, f64) {
    let t6658 = t6657 * t829;
    let t6660 = t235 * t6624;
    let t6662 = -t6636 - 0.16449340668482264365e-1_f64 * t6641 - t6645 - 0.82246703342411321825e-2_f64 * t6650 + 0.82246703342411321825e-2_f64 * t6654 + t808 * t1909 - t812 * t6658 + t226 * t6660;
    (t6658, t6660, t6662)
}
