//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta26 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk201;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk202;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk203;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk204;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk205;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk206;
use chunk6::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk207;
use chunk7::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk208;
use chunk8::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk209;
use chunk9::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk210;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta26<F: Float>(t530: F, t136: F, t221: F, t149: F, t225: F, t522: F, t524: F, t73: F, t235: F, t213: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t531 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk201::<F>(t530);
        let t532 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk202::<F>(t530, t531);
        let t535 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk203::<F>(t531, t136, t221);
        let t539 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk204::<F>(t149, t225, t522, t524);
        let (t540, t541) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk205::<F>(t532, t73);
        let t543 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk206::<F>(t539, t541);
        let (t544, t545) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk207::<F>(t543);
        let t546 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk208::<F>(t225, t545);
        let t547 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk209::<F>(t235, t546);
        let t548 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk210::<F>(t213, t547);
    (t531, t532, t535, t539, t540, t541, t543, t544, t545, t546, t547, t548)
}
