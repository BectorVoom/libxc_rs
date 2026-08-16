//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta870 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3029;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3030;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta870<F: Float>(t14742: F, t2689: F, t243: F, t9794: F, t10760: F, t14495: F, t14587: F, t40799: F, t4372: F, t9789: F, t40627: F, t50451: F, t50613: F, t14861: F, t10890: F, t4458: F, t10815: F, t4426: F, t40424: F, t4430: F, t14720: F, t9775: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t51074, t51078, t51081, t51083, t51086) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3029::<F>(t14742, t2689, t243, t9794, t10760, t14495, t14587, t40799, t4372, t9789, t40627, t50451);
        let (t51089, t51092, t51095, t51098, t51100, t51102) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3030::<F>(t10760, t40627, t50613, t14861, t9794, t10890, t4458, t10815, t4426, t40424, t4430, t14720, t9775);
    (t51074, t51078, t51081, t51083, t51086, t51089, t51092, t51095, t51098, t51100, t51102)
}
