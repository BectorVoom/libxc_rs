//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta882 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2792;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2793;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta882(t10090: f64, t122: f64, t14144: f64, t2482: f64, t6861: f64, t72: f64, t9994: f64, t14145: f64, t4114: f64, t10014: f64, t22336: f64, t1398: f64, t73820: f64, t2782: f64, t47371: f64, t6862: f64, t10022: f64, t22315: f64, t46457: f64, t136: f64, t2457: f64, t47429: f64, t22332: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t75035, t75039, t75041, t75047) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2792(t10090, t122, t14144, t2482, t6861, t72, t9994, t14145, t4114, t10014, t22336, t1398, t73820);
        let (t75049, t75053, t75060, t75068, t75071) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2793(t2782, t47371, t75047, t1398, t6862, t10022, t22315, t46457, t136, t2457, t47429, t10014, t22332);
    (t75035, t75039, t75041, t75049, t75053, t75060, t75068, t75071)
}
