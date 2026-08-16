//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 907/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk907(t3730: f64, t824: f64, t218: f64, t219: f64, t334: f64, t9795: f64, t6175: f64, t6177: f64, t7950: f64, t7980: f64, t7983: f64, t9812: f64, t9814: f64, t9819: f64, t9823: f64, t9826: f64) -> (f64, f64, f64, f64, f64) {
    let t9828 = t824 * t3730;
    let t9830 = t218 * t219 * t9828;
    let t9832 = t334 * t9795;
    let t9834 = t218 * t219 * t9832;
    let t9836 = 0.15358125e0_f64 * t9812 + 0.3071625e0_f64 * t9814 - t6175 + 0.27385555555555555556e0_f64 * t6177 + 0.5477111111111111111e0_f64 * t7950 - t7980 - t7983 - 0.16431333333333333333e0_f64 * t9819 + 0.49294e0_f64 * t9823 - 0.16431333333333333333e0_f64 * t9826 + 0.24647e0_f64 * t9830 + 0.24647e0_f64 * t9834;
    (t9828, t9830, t9832, t9834, t9836)
}
