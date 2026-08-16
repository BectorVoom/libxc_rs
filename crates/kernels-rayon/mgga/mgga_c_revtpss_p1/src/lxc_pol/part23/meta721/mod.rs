//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta721 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2482;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2483;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta721(t2482: f64, t814: f64, t9991: f64, t46917: f64, t5706: f64, t241: f64, t47201: f64, t820: f64, t47198: f64, t5665: f64, t5629: f64, t9779: f64, t5661: f64, t9909: f64, t47247: f64, t828: f64, t13941: f64, t46740: f64, t221: f64, t47273: f64, t13770: f64, t9775: f64, t40690: f64, t5610: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t48731, t48756, t48759, t48792, t48794) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2482(t2482, t814, t9991, t46917, t5706, t241, t47201, t820, t47198, t5665, t5629, t9779);
        let (t48797, t48798, t48814, t48823, t48827, t48829) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2483(t5661, t9909, t47247, t828, t13941, t46740, t221, t47273, t13770, t9775, t40690, t5610);
    (t48731, t48756, t48759, t48792, t48794, t48797, t48798, t48814, t48823, t48827, t48829)
}
