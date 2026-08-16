//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta10 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk80;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk81;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk82;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk83;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk84;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk85;
use chunk6::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk86;
use chunk7::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk87;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta10(t128: f64, t131: f64, t134: f64, t141: f64, t177: f64, t149: f64, t164: f64, t173: f64, t162: f64, t158: f64, t157: f64, t73: f64, t152: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t182, t185, t186) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk80(t128, t131, t134, t141);
        let t187 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk81(t177, t186);
        let t189 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk82(t149, t164, t173, t187);
        let t190 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk83(t162, t189);
        let (t191, t192) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk84(t158, t190, t157, t162);
        let (t194, t196) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk85(t187, t192);
        let t198 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk86(t73, t196);
        let t199 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk87(t152);
    (t182, t185, t186, t187, t189, t190, t191, t192, t194, t196, t198, t199)
}
