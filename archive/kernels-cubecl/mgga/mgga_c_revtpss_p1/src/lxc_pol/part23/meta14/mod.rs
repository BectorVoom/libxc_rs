//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta14 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk115;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk116;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk117;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk118;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk119;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk120;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk121;
use chunk7::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk122;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta14<F: Float>(t241: F, t243: F, t137: F, t72: F, t125: F, t217: F, t222: F, t237: F, t225: F, t234: F, t213: F, t149: F, t191: F, t194: F, t198: F, t207: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t244, t245) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk115::<F>(t241, t243, t137);
        let t246 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk116::<F>(t245, t72);
        let t247 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk117::<F>(t125, t246);
        let t251 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk118::<F>(t244, t247, t217, t222, t237);
        let t252 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk119::<F>(t225, t251);
        let (t253, t256, t257) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk120::<F>(t234, t251, t213);
        let (t258, t261, t262) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk121::<F>(t252, t257, t213);
        let t265 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk122::<F>(t149, t191, t194, t198, t207, t262);
    (t245, t246, t247, t251, t252, t253, t256, t257, t258, t261, t262, t265)
}
