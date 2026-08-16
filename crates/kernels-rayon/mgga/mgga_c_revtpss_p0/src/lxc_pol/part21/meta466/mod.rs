//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta466 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2010;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2011;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2012;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2013;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2014;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta466(t221: f64, t4433: f64, t10703: f64, t2674: f64, t4353: f64, t9794: f64, t10760: f64, t10890: f64, t1549: f64, t1544: f64, t2430: f64, t2477: f64, t828: f64, t2394: f64, t10698: f64, t10811: f64, t4462: f64, t4416: f64, t808: f64, t10886: f64, t2703: f64, t4458: f64, t10678: f64, t10682: f64, t10687: f64, t10692: f64, t851: f64, t10769: f64, t836: f64, t2749: f64, t2746: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14757, t14759, t14761, t14765, t14767) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2010(t221, t4433, t10703, t2674, t4353, t9794, t10760, t10890, t1549, t1544, t2430);
        let (t14769, t14772, t14774, t14777, t14779, t14780, t14783) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2011(t14767, t2477, t828, t1544, t2394, t10698, t10811, t4462, t4416, t808, t10886, t2703, t4458);
        let t14784 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2012(t10678, t10682, t10687, t10692, t14759, t14761, t14765, t14769, t14774, t14777, t14780, t14783, t851);
        let t14785 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2013(t10769, t828);
        let (t14786, t14787, t14788, t14791) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2014(t1544, t836, t2749, t14785, t2746, t828);
    (t14757, t14767, t14769, t14772, t14774, t14779, t14784, t14785, t14786, t14787, t14788, t14791)
}
