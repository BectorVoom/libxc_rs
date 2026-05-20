//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta920 (260520-c91 hierarchical CSE).
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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3131;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3132;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3133;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3134;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3135;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3136;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3137;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3138;
use chunk8::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3139;
use chunk9::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3140;
use chunk10::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3141;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta920<F: Float>(t342: F, t379: F, t2435: F, t5048: F, t5053: F, t16739: F, t689: F, t16743: F, t16734: F, t16726: F, t16730: F, t16721: F, t16716: F, t5057: F, t16747: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t56087, t56176) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3131::<F>(t342, t379, t2435, t5048);
        let t56183 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3132::<F>(t2435, t5053);
        let t56185 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3133::<F>(t16739, t689);
        let t56187 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3134::<F>(t16743, t689);
        let t56189 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3135::<F>(t16734, t689);
        let t56209 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3136::<F>(t16726, t689);
        let t56212 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3137::<F>(t16730, t689);
        let t56214 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3138::<F>(t16721, t689);
        let t56216 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3139::<F>(t16716, t689);
        let t56228 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3140::<F>(t2435, t5057);
        let t56230 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3141::<F>(t16747, t689);
    (t56087, t56176, t56183, t56185, t56187, t56189, t56209, t56212, t56214, t56216, t56228, t56230)
}
