//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta294 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1046;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1047;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1048;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta294<F: Float>(t10654: F, t2782: F, t2760: F, t822: F, t243: F, t816: F, t9707: F, t813: F, t2394: F, t2476: F, t236: F, t807: F, t2689: F, t2694: F, t2430: F, t854: F, t247: F, t9949: F, t237: F, t9646: F, t9721: F, t268: F, t2479: F, t2652: F, t207: F, t242: F, t240: F, t72: F, t136: F, t221: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t10655, t10657, t10673, t10676) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1046::<F>(t10654, t2782, t2760, t822, t243, t816, t9707, t813, t2394, t2476, t236, t807);
        let (t10678, t10682, t10687, t10688, t10689) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1047::<F>(t2689, t2694, t2430, t854, t236, t807, t243, t247, t9949, t237, t9646, t9721);
        let (t10692, t10693, t10698, t10703, t10705) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1048::<F>(t10689, t268, t10688, t2479, t2652, t207, t242, t240, t72, t136, t2476, t221, t2394);
    (t10655, t10657, t10673, t10676, t10678, t10682, t10687, t10692, t10693, t10698, t10703, t10705)
}
