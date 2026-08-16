//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 244/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk244(t12: f64, t135: f64, t273: f64, t661: f64, t687: f64, t727: f64, t729: f64, t734: f64, t803: f64, t805: f64, t439: f64, t204: f64, t334: f64, t648: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t84 = t12 <= zeta_threshold;
    let t808 = t135 * t273 * t803 * t805 - t661 + t687 + t727 + t729 - t734;
    let t810 = piecewise3(t84, 0.0_f64, t439);
    let t819 = t204 * t648 * t334;
    (t808, t810, t819)
}
