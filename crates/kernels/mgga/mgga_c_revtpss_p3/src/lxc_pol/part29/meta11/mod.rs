//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta11 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk77;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk78;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk79;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk80;
use chunk4::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk81;
use chunk5::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk82;
use chunk6::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk83;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta11<F: Float>(t158: F, t190: F, t157: F, t162: F, t187: F, t73: F, t152: F, t45: F, t57: F, t78: F, t81: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t191, t192, t194, t196) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk77::<F>(t158, t190, t157, t162, t187);
        let t197 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk78::<F>(t73);
        let t198 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk79::<F>(t196, t197);
        let t199 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk80::<F>(t152);
        let (t200, t202, t205) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk81::<F>(t45, t57, t78, t199, t81, zeta_threshold);
        let t206 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk82::<F>(t205);
        let t207 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk83::<F>(t205, t206);
    (t191, t192, t194, t196, t197, t198, t199, t200, t202, t205, t206, t207)
}
