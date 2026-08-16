//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 253/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk253(t836: f64, t841: f64, t218: f64, t344: f64, t675: f64, t334: f64, t824: f64, t219: f64, t826: f64, t837: f64, t839: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t842 = t841 * t836;
    let t845 = t218 * t675 * t344;
    let t846 = 0.82156666666666666667e-1_f64 * t845;
    let t847 = t334 * t824;
    let t849 = t218 * t219 * t847;
    let t851 = 0.1898925e1_f64 * t837 - t839 + 0.8969e0_f64 * t826 + 0.3071625e0_f64 * t842 - t846 + 0.24647e0_f64 * t849;
    (t842, t845, t846, t847, t849, t851)
}
