//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta382 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1947;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1948;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta382<F: Float>(t13581: F, t187: F, t1857: F, t3857: F, t5591: F, t566: F, t9375: F, t177: F, t5566: F, t762: F, t1450: F, t5778: F, t3889: F, t5537: F, t1353: F, t1868: F, t3829: F, t4139: F, t5532: F, t5536: F, t9278: F, t9308: F, t9316: F, t9320: F, t9325: F, t9329: F, t9333: F, t9374: F, t9389: F, t9391: F, t9547: F, t9599: F) -> (F, F, F, F, F, F, F, F) {
        let (t13583, t13584, t13585, t13586, t13593, t13597, t13599, t13600) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1947::<F>(t13581, t187, t1857, t3857, t5591, t566, t9375, t177, t5566, t762, t1450, t5778);
        let t13610 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1948::<F>(t3889, t5537, t1353, t13583, t13585, t13586, t13593, t13599, t13600, t1868, t3829, t4139, t5532, t5536, t9278, t9308, t9316, t9320, t9325, t9329, t9333, t9374, t9389, t9391, t9547, t9599);
    (t13583, t13584, t13585, t13593, t13597, t13599, t13600, t13610)
}
