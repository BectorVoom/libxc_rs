//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta239 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1494;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1495;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta239<F: Float>(t5883: F, t94: F, t1518: F, t1843: F, t1513: F, t2339: F, t1504: F, t2349: F, t100: F, t5823: F, t1479: F, t1509: F, tau1: F, t2357: F, t108: F, t105: F, t109: F, t1507: F, t1510: F, t97: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t5884, t5887, t5891, t5892, t5895, t5896, t5899, t5902, t5907) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1494::<F>(t5883, t94, t1518, t1843, t1513, t2339, t1504, t2349, t100, t5823, t1479, t1509, tau1);
        let (t5911, t5915) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1495::<F>(t2357, t5907, t5823, t108, t105, t109, t1507, t1510, t5896, t5899, t5902, t97);
    (t5884, t5887, t5891, t5892, t5895, t5896, t5899, t5902, t5907, t5911, t5915)
}
