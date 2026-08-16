//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta13 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk103;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk104;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk105;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk106;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk107;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk108;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk109;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk110;
use chunk8::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk111;
use chunk9::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk112;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta13(t231: f64, t225: f64, t64: f64, t213: f64, t21: f64, t66: f64, t159: f64, t206: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t232, t233) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk103(t231);
        let t234 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk104(t225, t233);
        let t235 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk105(t64);
        let t236 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk106(t234, t235);
        let t237 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk107(t213, t236);
        let t239 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk108(t21, t66);
        let t240 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk109(t159);
        let t241 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk110(t239, t240);
        let t242 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk111(t206);
        let t243 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk112(t242);
    (t232, t233, t234, t235, t236, t237, t239, t240, t241, t242, t243)
}
