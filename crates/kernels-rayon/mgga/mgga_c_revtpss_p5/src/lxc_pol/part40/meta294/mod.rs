//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta294 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1049;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1050;
use chunk2::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1051;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta294(t10654: f64, t2782: f64, t2760: f64, t822: f64, t243: f64, t816: f64, t9707: f64, t813: f64, t2394: f64, t2476: f64, t236: f64, t807: f64, t2689: f64, t2694: f64, t2430: f64, t854: f64, t247: f64, t9949: f64, t237: f64, t9646: f64, t9721: f64, t268: f64, t2479: f64, t2652: f64, t207: f64, t242: f64, t240: f64, t72: f64, t136: f64, t221: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10655, t10657, t10673, t10676) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1049(t10654, t2782, t2760, t822, t243, t816, t9707, t813, t2394, t2476, t236, t807);
        let (t10678, t10682, t10687, t10688, t10689) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1050(t2689, t2694, t2430, t854, t236, t807, t243, t247, t9949, t237, t9646, t9721);
        let (t10692, t10693, t10698, t10703, t10705) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1051(t10689, t268, t10688, t2479, t2652, t207, t242, t240, t72, t136, t2476, t221, t2394);
    (t10655, t10657, t10673, t10676, t10678, t10682, t10687, t10692, t10693, t10698, t10703, t10705)
}
