//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta12 (260520-c91 hierarchical CSE).
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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk96;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk97;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk98;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk99;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk100;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk101;
use chunk6::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk102;
use chunk7::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk103;
use chunk8::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk104;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta12(t159: f64, t216: f64, t206: f64, t122: f64, t124: f64, t136: f64, t196: f64, t149: f64, t191: f64, t194: f64, t207: f64, t73: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t217, t218, t220) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk96(t159, t216, t206, t122);
        let t221 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk97(t124, t220);
        let t222 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk98(t136, t218, t221);
        let t225 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk99(t196);
        let t227 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk100(t149, t191, t194, t225);
        let (t228, t229) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk101(t207, t73);
        let t231 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk102(t227, t229);
        let (t232, t233) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk103(t231);
        let t234 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk104(t225, t233);
    (t217, t220, t221, t222, t225, t227, t228, t229, t231, t232, t233, t234)
}
