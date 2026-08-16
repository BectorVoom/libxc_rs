//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta277 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1241;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1242;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1243;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1244;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1245;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta277(t1916: f64, t2042: f64, t1518: f64, t7330: f64, t572: f64, t117: f64, t7741: f64, t1918: f64, t2040: f64, t573: f64, t7944: f64, t3140: f64, t3268: f64, t1078: f64, t1035: f64, t2033: f64, t4147: f64, t587: f64, t65: f64, t197: f64, t532: f64, t1450: f64, t143: f64, t2580: f64, t130: f64, t2566: f64, t700: f64, t2584: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t7950, t7953, t7956, t8515) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1241(t1916, t2042, t1518, t7330, t572, t117, t7741, t1918, t2040, t573, t7944, t3140, t3268);
        let t8521 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1242(t1078, t3140, t1035);
        let t8717 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1243(t2033, t4147);
        let (t8779, t8995, t8996, t9274, t9275, t9276) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1244(t587, t65, t197, t532, t1450, t2033, t143, t2580, t130, t2566, t700, t2584);
        let t9278 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1245(t9274, t9276);
    (t7950, t7953, t7956, t8515, t8521, t8717, t8779, t8995, t8996, t9275, t9278)
}
