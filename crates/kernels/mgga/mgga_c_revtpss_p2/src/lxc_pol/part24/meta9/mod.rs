//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta9 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk72;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk73;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk74;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk75;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk76;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk77;
use chunk6::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk78;
use chunk7::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk79;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta9<F: Float>(t45: F, zeta_threshold: F, t79: F, t57: F, t82: F, t150: F, t128: F, t131: F, t134: F, t141: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t152, t153) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk72::<F>(t45, zeta_threshold);
        let t157 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk73::<F>(t45, t153, t79, t57, t82, zeta_threshold);
        let t158 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk74::<F>(t150, t157);
        let t159 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk75::<F>();
        let t162 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk76::<F>(t159);
        let t164 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk77::<F>(t128);
        let (t169, t172, t173) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk78::<F>(t128, t131, t134, t141);
        let t177 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk79::<F>(t128);
    (t152, t153, t157, t158, t159, t162, t164, t169, t172, t173, t177)
}
