//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3020/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3020(t10811: f64, t14919: f64, t14904: f64, t14923: f64, t241: f64, t40322: f64, t820: f64, t2659: f64, t2783: f64, t816: f64, t808: f64, t853: f64) -> (f64, f64, f64, f64, f64) {
    let t50752 = t10811 * t14919;
    let t50754 = t14923 * t14904;
    let t50757 = t820 * t40322 * t241;
    let t50768 = t816 * t2659 * t2783;
    let t50769 = t808 * t853;
    (t50752, t50754, t50757, t50768, t50769)
}
