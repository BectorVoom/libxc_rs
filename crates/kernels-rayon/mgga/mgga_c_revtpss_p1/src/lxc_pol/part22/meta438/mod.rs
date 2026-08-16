//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta438 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2070;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2071;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2072;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta438(t240: f64, t849: f64, t14648: f64, t775: f64, t2661: f64, t2652: f64, t4345: f64, t10716: f64, t4349: f64, t10746: f64, t10749: f64, t10756: f64, t10758: f64, t14817: f64, t14820: f64, t14823: f64, t14825: f64, t14829: f64, t2730: f64, t1548: f64, t2394: f64, t800: f64, t2689: f64, t4372: f64, t4354: f64, t9775: f64, t14468: f64, t828: f64, t855: f64, t221: f64, t2675: f64, t4343: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t14832 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2070(t240, t849);
        let (t14833, t14834, t14836, t14837, t14839, t14841) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2071(t14648, t775, t14832, t2661, t2652, t4345, t10716, t4349, t10746, t10749, t10756, t10758, t14817, t14820, t14823, t14825, t14829, t2730);
        let (t14843, t14846, t14850, t14853, t14857) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2072(t1548, t2394, t800, t2689, t4372, t4354, t9775, t14468, t828, t855, t221, t2675, t4343);
    (t14832, t14833, t14834, t14836, t14837, t14839, t14841, t14843, t14846, t14850, t14853, t14857)
}
