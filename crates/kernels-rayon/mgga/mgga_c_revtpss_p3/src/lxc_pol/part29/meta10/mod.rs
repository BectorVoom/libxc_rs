//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta10 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk72;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk73;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk74;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk75;
use chunk4::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk76;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta10(t128: f64, t131: f64, t134: f64, t141: f64, t149: f64, t164: f64, t162: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t169, t172, t173, t177) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk72(t128, t131, t134, t141);
        let (t182, t185, t186) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk73(t128, t131, t134, t141);
        let t187 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk74(t177, t186);
        let t189 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk75(t149, t164, t173, t187);
        let t190 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk76(t162, t189);
    (t169, t172, t173, t177, t182, t185, t186, t187, t189, t190)
}
