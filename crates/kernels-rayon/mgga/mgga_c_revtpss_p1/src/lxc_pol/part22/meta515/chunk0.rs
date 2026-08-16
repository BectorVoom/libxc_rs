//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2280/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2280(t16831: f64, t448: f64, t300: f64, t1130: f64, t5060: f64, t1151: f64, t3428: f64, t5063: f64, t1719: f64, t3432: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16832 = t16831 * t448;
    let t16834 = 0.19751673498613801407e-1_f64 * t300 * t16832;
    let t16835 = t5060 * t1130;
    let t16837 = 2.0_f64 * t16835 * t1151;
    let t16839 = 1.0_f64 * t5063 * t3428;
    let t16840 = t1719 * t3432;
    (t16832, t16834, t16835, t16837, t16839, t16840)
}
