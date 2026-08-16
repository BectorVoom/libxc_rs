//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta693 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2438;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2439;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta693(t549: f64, t240: f64, t72: f64, t1408: f64, t2237: f64, t2482: f64, t3981: f64, t1369: f64, t9726: f64, t1372: f64, t546: f64, t9801: f64, t9738: f64, t794: f64, t9747: f64, t2699: f64, t3943: f64, t3995: f64, t40690: f64, t136: f64, t9941: f64, t1386: f64, t820: f64, t9948: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t46627, t46644, t46645, t46651, t46652, t46670) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2438(t549, t240, t72, t1408, t2237, t2482, t3981, t1369, t9726, t1372, t546, t9801);
        let (t46671, t46691, t46694, t46702, t46716, t46722) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2439(t46670, t9738, t794, t9747, t2699, t3943, t3995, t40690, t136, t9941, t1386, t820, t9948);
    (t46627, t46644, t46645, t46651, t46652, t46670, t46671, t46691, t46694, t46702, t46716, t46722)
}
