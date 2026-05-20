//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta112 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk647;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk648;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk649;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk650;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk651;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk652;
use chunk6::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk653;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta112<F: Float>(t1045: F, t3133: F, t373: F, t1042: F, t1031: F, t196: F, t342: F, t1034: F, t358: F, t360: F, t368: F, t335: F, t365: F, t1043: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3135, t3136) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk647::<F>(t1045, t3133, t373, t1042);
        let t3140 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk648::<F>(t1031, t196);
        let t3141 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk649::<F>(t3140, t342);
        let (t3143, t3144) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk650::<F>(t1034, t358, t360);
        let t3145 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk651::<F>(t368);
        let t3147 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk652::<F>(t3145, t335);
        let (t3148, t3149, t3150, t3151) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk653::<F>(t3147, t365, t3144, t3141, t1043);
    (t3135, t3136, t3140, t3141, t3143, t3144, t3145, t3147, t3148, t3149, t3150, t3151)
}
