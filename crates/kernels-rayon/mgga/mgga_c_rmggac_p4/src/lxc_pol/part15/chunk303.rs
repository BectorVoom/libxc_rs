//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 303/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk303(t53: f64, t60: f64, t1818: f64, t196: f64, t1794: f64, t1797: f64, t437: f64, t983: f64, t1802: f64, t1805: f64, t441: f64, t990: f64, zeta_threshold: f64) -> (f64, f64) {
    let t54 = t53 <= zeta_threshold;
    let t61 = t60 <= zeta_threshold;
    let t1819 = t196 * t1818;
    let t1827 = piecewise3(t54, 0.0_f64, -2.0_f64 / 9.0_f64 * t983 * t1794 + 2.0_f64 / 3.0_f64 * t437 * t1797);
    let t1833 = piecewise3(t61, 0.0_f64, -2.0_f64 / 9.0_f64 * t990 * t1802 + 2.0_f64 / 3.0_f64 * t441 * t1805);
    let t1835 = t1827 / 2.0_f64 + t1833 / 2.0_f64;
    (t1819, t1835)
}
