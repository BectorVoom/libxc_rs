//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1211/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1211<F: Float>(t105420: F, t105558: F, t112758: F, t116356: F, t1774: F, t1811: F, t2152: F, t24519: F, t24892: F, t26949: F, t26969: F, t26976: F, t29129: F, t29136: F, t29141: F, t29304: F, t30740: F, t30748: F, t30764: F, t30772: F, t30840: F, t30853: F, t30882: F, t30883: F, t30893: F, t6587: F, t6588: F, t6702: F, t7602: F, t7637: F, t7643: F, t7651: F, t8190: F, t8201: F, t8209: F, t8217: F, t97304: F) -> (F,) {
    let t116469 = -0.78062653693846795158e1 * t105420 * t30740 - 0.39512695097613069591e1 * t7602 * t24519 + 0.10408353825846239354e2 * t97304 * t30853 * t116356 - 0.26020884564615598386e1 * t30883 * t8217 - 0.26020884564615598386e1 * t30882 * t1811 * t2152 - 0.78062653693846795158e1 * t26949 * t7637 * t8201 * t6587 + 0.39512695097613069591e1 * t26976 * t24892 + 0.26020884564615598386e1 * t7643 * t7637 * t30840 * t1774 + 0.26020884564615598386e1 * t112758 * t8209 - 0.78062653693846795158e1 * t7651 * t26969 * t8190 * t6702 + 0.52041769129231196772e1 * t29136 * t30748 - 0.19756347548806534796e1 * t29304 * t6588 + 0.26020884564615598386e1 * t29141 * t30772 - 0.26020884564615598386e1 * t29129 * t30893 + 0.52041769129231196772e1 * t105558 * t30764;
    (t116469,)
}
