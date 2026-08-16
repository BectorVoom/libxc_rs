//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3026/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3026(t241: f64, t820: f64, t849: f64, t14900: f64, t14923: f64, t10811: f64, t14914: f64, t14788: f64, t10886: f64, t14652: f64, t808: f64, t14746: f64, t2703: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t50957 = t820 * t849 * t241;
    let t50966 = t14923 * t14900;
    let t50968 = t10811 * t14914;
    let t50974 = t10811 * t14788;
    let t50977 = t10886 * t808 * t14652;
    let t50982 = t2703 * t14746;
    (t50957, t50966, t50968, t50974, t50977, t50982)
}
