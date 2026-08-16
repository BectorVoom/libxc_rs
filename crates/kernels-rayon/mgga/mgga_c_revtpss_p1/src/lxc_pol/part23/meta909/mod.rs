//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta909 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2919;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2920;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2921;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2922;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta909(t141: f64, t77579: f64, t930: f64, t2908: f64, t77584: f64, t11341: f64, t77564: f64, t77568: f64, t41294: f64, t77573: f64, t42731: f64, t52011: f64, t77513: f64, t42518: f64, t41307: f64, t63276: f64, t63278: f64, t77507: f64, t77509: f64, t23495: f64, t698: f64, t52018: f64, t41361: f64, t51974: f64, t51978: f64, t63320: f64, t77515: f64, t77518: f64, t77521: f64, t77527: f64, t77531: f64, t77535: f64, t41329: f64, t52082: f64, t77499: f64, t77503: f64, t77505: f64, t77539: f64, t77543: f64, t77547: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t77712, t77715, t77718, t77721, t77724, t77727) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2919(t141, t77579, t930, t2908, t77584, t11341, t77564, t77568, t41294, t77573, t42731, t52011, t77513);
        let (t77730, t77732) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2920(t42518, t52011, t77513, t41307, t63276, t63278, t77507, t77509, t77712, t77715, t77718, t77721, t77724, t77727);
        let (t77736, t77739, t77747) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2921(t23495, t698, t52011, t52018, t77513, t41361, t51974, t51978, t63320, t77515, t77518, t77521, t77527, t77531, t77535);
        let t77778 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2922(t41329, t41361, t51978, t52082, t63276, t63278, t77499, t77503, t77505, t77507, t77509, t77515, t77518, t77521, t77527, t77531, t77535, t77539, t77543, t77547);
    (t77712, t77715, t77718, t77721, t77724, t77727, t77730, t77732, t77736, t77739, t77747, t77778)
}
