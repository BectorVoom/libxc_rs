//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta421 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1910;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1911;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta421<F: Float>(t3889: F, t5537: F, t1353: F, t13583: F, t13585: F, t13586: F, t13593: F, t13599: F, t13600: F, t1868: F, t3829: F, t4139: F, t5532: F, t5536: F, t9278: F, t9308: F, t9316: F, t9320: F, t9325: F, t9329: F, t9333: F, t9374: F, t9389: F, t9391: F, t9547: F, t9599: F, t2516: F, t5571: F, t5566: F, t72: F, t757: F, t1320: F, t5567: F, t5569: F, t9395: F, t9398: F, t1448: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t13607, t13610) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1910::<F>(t3889, t5537, t1353, t13583, t13585, t13586, t13593, t13599, t13600, t1868, t3829, t4139, t5532, t5536, t9278, t9308, t9316, t9320, t9325, t9329, t9333, t9374, t9389, t9391, t9547, t9599);
        let (t13612, t13613, t13615, t13620, t13622, t13623, t13624, t13625) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1911::<F>(t2516, t5571, t5566, t72, t757, t1320, t5567, t5569, t9395, t9398, t1353, t1448);
    (t13607, t13610, t13612, t13613, t13615, t13620, t13622, t13623, t13624, t13625)
}
