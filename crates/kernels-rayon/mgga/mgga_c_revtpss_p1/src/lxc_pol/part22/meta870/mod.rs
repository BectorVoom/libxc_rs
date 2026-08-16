//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta870 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3029;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3030;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta870(t14742: f64, t2689: f64, t243: f64, t9794: f64, t10760: f64, t14495: f64, t14587: f64, t40799: f64, t4372: f64, t9789: f64, t40627: f64, t50451: f64, t50613: f64, t14861: f64, t10890: f64, t4458: f64, t10815: f64, t4426: f64, t40424: f64, t4430: f64, t14720: f64, t9775: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t51074, t51078, t51081, t51083, t51086) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3029(t14742, t2689, t243, t9794, t10760, t14495, t14587, t40799, t4372, t9789, t40627, t50451);
        let (t51089, t51092, t51095, t51098, t51100, t51102) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3030(t10760, t40627, t50613, t14861, t9794, t10890, t4458, t10815, t4426, t40424, t4430, t14720, t9775);
    (t51074, t51078, t51081, t51083, t51086, t51089, t51092, t51095, t51098, t51100, t51102)
}
