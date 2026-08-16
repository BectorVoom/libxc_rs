//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1361/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1361(t105420: f64, t105558: f64, t112758: f64, t116356: f64, t1774: f64, t1811: f64, t2152: f64, t24519: f64, t24892: f64, t26949: f64, t26969: f64, t26976: f64, t29129: f64, t29136: f64, t29141: f64, t29304: f64, t30740: f64, t30748: f64, t30764: f64, t30772: f64, t30840: f64, t30853: f64, t30882: f64, t30883: f64, t30893: f64, t6587: f64, t6588: f64, t6702: f64, t7602: f64, t7637: f64, t7643: f64, t7651: f64, t8190: f64, t8201: f64, t8209: f64, t8217: f64, t97304: f64) -> f64 {
    let t116469 = -0.78062653693846795158e1_f64 * t105420 * t30740 - 0.39512695097613069591e1_f64 * t7602 * t24519 + 0.10408353825846239354e2_f64 * t97304 * t30853 * t116356 - 0.26020884564615598386e1_f64 * t30883 * t8217 - 0.26020884564615598386e1_f64 * t30882 * t1811 * t2152 - 0.78062653693846795158e1_f64 * t26949 * t7637 * t8201 * t6587 + 0.39512695097613069591e1_f64 * t26976 * t24892 + 0.26020884564615598386e1_f64 * t7643 * t7637 * t30840 * t1774 + 0.26020884564615598386e1_f64 * t112758 * t8209 - 0.78062653693846795158e1_f64 * t7651 * t26969 * t8190 * t6702 + 0.52041769129231196772e1_f64 * t29136 * t30748 - 0.19756347548806534796e1_f64 * t29304 * t6588 + 0.26020884564615598386e1_f64 * t29141 * t30772 - 0.26020884564615598386e1_f64 * t29129 * t30893 + 0.52041769129231196772e1_f64 * t105558 * t30764;
    t116469
}
