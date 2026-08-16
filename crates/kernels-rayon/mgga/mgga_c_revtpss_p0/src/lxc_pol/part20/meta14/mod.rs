//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta14 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk117;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk118;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk119;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk120;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk121;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk122;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta14(t225: f64, t251: f64, t234: f64, t213: f64, t149: f64, t191: f64, t194: f64, t198: f64, t207: f64, t123: f64, t125: f64, t126: f64, t159: f64, t45: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t252 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk117(t225, t251);
        let (t253, t256, t257) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk118(t234, t251, t213);
        let (t258, t261, t262) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk119(t252, t257, t213);
        let t265 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk120(t149, t191, t194, t198, t207, t262);
        let t268 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk121(t123, t125);
        let (t269, t270, t271) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk122(t126, t159, t45);
    (t252, t253, t256, t257, t258, t261, t262, t265, t268, t269, t270, t271)
}
