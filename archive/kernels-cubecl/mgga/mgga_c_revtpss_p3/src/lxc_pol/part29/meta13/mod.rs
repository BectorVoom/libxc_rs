//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta13 (260520-c91 hierarchical CSE).
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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk93;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk94;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk95;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk96;
use chunk4::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk97;
use chunk5::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk98;
use chunk6::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk99;
use chunk7::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk100;
use chunk8::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk101;
use chunk9::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk102;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta13<F: Float>(t149: F, t191: F, t194: F, t225: F, t207: F, t73: F, t64: F, t213: F, t21: F, t66: F, t159: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t227 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk93::<F>(t149, t191, t194, t225);
        let t228 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk94::<F>(t207);
        let t229 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk95::<F>(t228, t73);
        let t231 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk96::<F>(t227, t229);
        let (t232, t233) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk97::<F>(t231);
        let t234 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk98::<F>(t225, t233);
        let t235 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk99::<F>(t64);
        let t236 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk100::<F>(t234, t235);
        let (t237, t239) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk101::<F>(t213, t236, t21, t66);
        let t240 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk102::<F>(t159);
    (t227, t228, t229, t231, t232, t233, t234, t235, t236, t237, t239, t240)
}
