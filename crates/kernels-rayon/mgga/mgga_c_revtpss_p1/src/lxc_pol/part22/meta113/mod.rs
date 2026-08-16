//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta113 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk766;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk767;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk768;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk769;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk770;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta113(t225: f64, t2735: f64, t826: f64, t849: f64, t820: f64, t823: f64, t843: f64, t839: f64, t241: f64, t72: f64, t853: f64, t245: f64, t231: f64, t775: f64, t125: f64, t836: f64, t2722: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2736, t2737, t2739, t2741) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk766(t225, t2735, t826, t849, t820, t823, t843);
        let (t2742, t2745) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk767(t2741, t839, t241, t820, t823);
        let (t2746, t2747) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk768(t72, t853, t245);
        let t2749 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk769(t231, t775);
        let (t2751, t2754) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk770(t125, t2749, t836, t2747, t231, t2722);
    (t2736, t2737, t2739, t2741, t2742, t2745, t2746, t2747, t2749, t2751, t2754)
}
