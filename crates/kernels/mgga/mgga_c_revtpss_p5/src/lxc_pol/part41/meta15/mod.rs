//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta15 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk102;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk103;
use chunk2::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk104;
use chunk3::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk105;
use chunk4::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk106;
use chunk5::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk107;
use chunk6::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk108;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta15<F: Float>(t252: F, t257: F, t213: F, t149: F, t191: F, t194: F, t198: F, t207: F, t123: F, t125: F, t126: F, t159: F, t45: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t258, t261, t262) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk102::<F>(t252, t257, t213);
        let t265 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk103::<F>(t149, t191, t194, t198, t207, t262);
        let t268 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk104::<F>(t123, t125);
        let (t269, t270, t271) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk105::<F>(t126, t159, t45);
        let t273 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk106::<F>(t268, t269, t271);
        let t275 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk107::<F>(t273);
        let t276 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk108::<F>(t273);
    (t258, t261, t262, t265, t268, t269, t270, t271, t273, t275, t276)
}
