//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta27 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk207;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk208;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk209;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk210;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk211;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk212;
use chunk6::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk213;
use chunk7::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk214;
use chunk8::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk215;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta27<F: Float>(t543: F, t225: F, t235: F, t213: F, t531: F, t241: F, t247: F, t217: F, t535: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t544, t545) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk207::<F>(t543);
        let t546 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk208::<F>(t225, t545);
        let t547 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk209::<F>(t235, t546);
        let t548 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk210::<F>(t213, t547);
        let t549 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk211::<F>(t531);
        let t550 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk212::<F>(t549);
        let t555 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk213::<F>(t241, t550, t247, t217, t535, t548);
        let t556 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk214::<F>(t225, t555);
        let (t557, t560, t561) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk215::<F>(t546, t555, t213);
    (t544, t545, t546, t547, t548, t549, t550, t555, t556, t557, t560, t561)
}
