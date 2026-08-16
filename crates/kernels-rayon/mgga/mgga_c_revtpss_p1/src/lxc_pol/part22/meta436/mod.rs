//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta436 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2064;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2065;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2066;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta436(t1544: f64, t2394: f64, t10698: f64, t828: f64, t10811: f64, t4462: f64, t4416: f64, t808: f64, t10886: f64, t2703: f64, t4458: f64, t10678: f64, t10682: f64, t10687: f64, t10692: f64, t14759: f64, t14761: f64, t14765: f64, t14769: f64, t851: f64, t10769: f64, t836: f64, t2749: f64, t2746: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14772, t14774, t14777, t14779, t14780, t14783, t14784) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2064(t1544, t2394, t10698, t828, t10811, t4462, t4416, t808, t10886, t2703, t4458, t10678, t10682, t10687, t10692, t14759, t14761, t14765, t14769, t851);
        let t14785 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2065(t10769, t828);
        let (t14786, t14787, t14788, t14791) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2066(t1544, t836, t2749, t14785, t2746, t828);
    (t14772, t14774, t14777, t14779, t14780, t14783, t14784, t14785, t14786, t14787, t14788, t14791)
}
