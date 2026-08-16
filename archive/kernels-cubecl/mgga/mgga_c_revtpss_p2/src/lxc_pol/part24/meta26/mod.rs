//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta26 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk200;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk201;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk202;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk203;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk204;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk205;
use chunk6::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk206;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta26<F: Float>(t30: F, t33: F, t512: F, t521: F, t187: F, t520: F, t513: F, t199: F, t516: F, zeta_threshold: F, t136: F, t221: F, t149: F, t225: F, t73: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t522, t524, t525, t527, t530) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk200::<F>(t30, t33, t512, t521, t187, t520, t513, t199, t516, zeta_threshold);
        let t531 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk201::<F>(t530);
        let t532 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk202::<F>(t530, t531);
        let t535 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk203::<F>(t531, t136, t221);
        let t539 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk204::<F>(t149, t225, t522, t524);
        let (t540, t541) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk205::<F>(t532, t73);
        let t543 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk206::<F>(t539, t541);
    (t522, t524, t525, t527, t530, t531, t532, t535, t539, t540, t541, t543)
}
