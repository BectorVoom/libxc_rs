//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta420 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1907;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1908;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1909;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta420<F: Float>(t33: F, t1711: F, t9350: F, t2: F, t3841: F, t1113: F, t580: F, t22: F, t3351: F, t3842: F, t516: F, t5557: F, t5560: F, zeta_threshold: F, t13564: F, t162: F, t187: F, t1857: F, t3857: F, t5591: F, t566: F, t9375: F, t177: F, t5566: F, t762: F, t1450: F, t5778: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t13565, t13568, t13569, t13579) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1907::<F>(t33, t1711, t9350, t2, t3841, t1113, t580, t22, t3351, t3842, t516, t5557, t5560, zeta_threshold);
        let t13581 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1908::<F>(t13564, t13579, t162);
        let (t13583, t13585, t13586, t13593, t13597, t13599, t13600) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1909::<F>(t13581, t187, t1857, t3857, t5591, t566, t9375, t177, t5566, t762, t1450, t5778);
    (t13565, t13568, t13569, t13581, t13583, t13585, t13586, t13593, t13597, t13599, t13600)
}
