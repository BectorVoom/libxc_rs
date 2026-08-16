//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta13 (260520-c91 hierarchical CSE).
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
mod chunk10;
mod chunk11;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk105;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk106;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk107;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk108;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk109;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk110;
use chunk6::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk111;
use chunk7::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk112;
use chunk8::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk113;
use chunk9::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk114;
use chunk10::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk115;
use chunk11::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk116;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta13<F: Float>(t64: F, t234: F, t213: F, t21: F, t66: F, t159: F, t206: F, t137: F, t72: F, t125: F, t217: F, t222: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t235 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk105::<F>(t64);
        let t236 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk106::<F>(t234, t235);
        let t237 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk107::<F>(t213, t236);
        let t239 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk108::<F>(t21, t66);
        let t240 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk109::<F>(t159);
        let t241 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk110::<F>(t239, t240);
        let t242 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk111::<F>(t206);
        let t243 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk112::<F>(t242);
        let (t244, t245) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk113::<F>(t241, t243, t137);
        let t246 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk114::<F>(t245, t72);
        let t247 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk115::<F>(t125, t246);
        let t251 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk116::<F>(t244, t247, t217, t222, t237);
    (t235, t236, t237, t239, t240, t241, t242, t243, t245, t246, t247, t251)
}
