//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta712 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2544;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2545;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta712(t3946: f64, t46694: f64, t3995: f64, t40690: f64, t9775: f64, t9936: f64, t3970: f64, t9779: f64, t9765: f64, t9923: f64, t136: f64, t9941: f64, t221: f64, t3978: f64, t9400: f64, t1386: f64, t820: f64, t9948: f64, t1401: f64, t159: f64, t216: f64, t4010: f64, t2482: f64, t2668: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t46695, t46702, t46704, t46706, t46712, t46716) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2544(t3946, t46694, t3995, t40690, t9775, t9936, t3970, t9779, t9765, t9923, t136, t9941);
        let (t46719, t46722, t46723, t46730, t46740) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2545(t221, t3978, t46716, t9400, t1386, t820, t9948, t1401, t159, t216, t4010, t2482, t2668);
    (t46695, t46702, t46704, t46706, t46712, t46716, t46719, t46722, t46723, t46730, t46740)
}
