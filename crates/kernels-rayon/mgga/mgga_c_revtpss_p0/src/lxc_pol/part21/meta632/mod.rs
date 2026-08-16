//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta632 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2400;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2401;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta632(t40861: f64, t802: f64, t10899: f64, t794: f64, t10902: f64, t159: f64, t216: f64, t2475: f64, t2645: f64, t860: f64, t231: f64, t2782: f64, t2783: f64, t39714: f64, t2723: f64, t39704: f64, t4503: f64, t123: f64, t212: f64, t9291: f64, t2786: f64, t10073: f64, t10654: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40862, t40864, t40865, t40868, t40888, t40894) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2400(t40861, t802, t10899, t794, t10902, t159, t216, t2475, t2645, t860, t231, t2782, t2783, t39714);
        let (t40914, t40918, t40921, t40922, t40924) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2401(t231, t2782, t2783, t40888, t2723, t39704, t4503, t123, t212, t9291, t2786, t10073, t10654);
    (t40862, t40864, t40865, t40868, t40894, t40914, t40918, t40921, t40922, t40924)
}
