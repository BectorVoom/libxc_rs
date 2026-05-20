//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta38 (260520-c91 hierarchical CSE).
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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk273;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk274;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk275;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk276;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk277;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk278;
use chunk6::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk279;
use chunk7::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk280;
use chunk8::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk281;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta38<F: Float>(t240: F, t243: F, t72: F, t125: F, t245: F, t73: F, t587: F, t66: F, t247: F, t237: F, t233: F, t235: F, t239: F, t820: F, t205: F, t242: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t826 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk273::<F>(t240, t243);
        let t827 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk274::<F>(t72, t826);
        let t828 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk275::<F>(t125, t245);
        let (t832, t843) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk276::<F>(t243, t73, t587, t66);
        let t844 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk277::<F>(t240, t843);
        let (t848, t849) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk278::<F>(t243, t844, t247, t237, t233, t235);
        let t851 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk279::<F>(t239, t820, t849);
        let t853 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk280::<F>(t205, t242);
        let t854 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk281::<F>(t240, t853);
    (t826, t827, t828, t832, t843, t844, t848, t849, t851, t853, t854)
}
