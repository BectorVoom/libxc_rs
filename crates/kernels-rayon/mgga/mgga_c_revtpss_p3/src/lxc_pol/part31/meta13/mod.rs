//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta13 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;
mod chunk9;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk95;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk96;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk97;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk98;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk99;
use chunk5::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk100;
use chunk6::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk101;
use chunk7::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk102;
use chunk8::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk103;
use chunk9::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk104;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta13(t149: f64, t191: f64, t194: f64, t225: f64, t207: f64, t73: f64, t64: f64, t213: f64, t21: f64, t66: f64, t159: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t227 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk95(t149, t191, t194, t225);
        let t228 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk96(t207);
        let t229 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk97(t228, t73);
        let t231 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk98(t227, t229);
        let (t232, t233) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk99(t231);
        let t234 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk100(t225, t233);
        let t235 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk101(t64);
        let t236 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk102(t234, t235);
        let (t237, t239) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk103(t213, t236, t21, t66);
        let t240 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk104(t159);
    (t227, t228, t229, t231, t232, t233, t234, t235, t236, t237, t239, t240)
}
