//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta207 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1229;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1230;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta207<F: Float>(t5883: F, t94: F, t1518: F, t1843: F, t1513: F, t2339: F, t1504: F, t2349: F, t100: F, t5823: F, t1479: F, tau1: F, t1509: F, t2357: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t5884, t5887, t5891, t5892, t5895, t5896, t5899, t5902) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1229::<F>(t5883, t94, t1518, t1843, t1513, t2339, t1504, t2349, t100, t5823, t1479, tau1);
        let (t5907, t5908, t5911) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1230::<F>(t1509, t2357, t5823);
    (t5884, t5887, t5891, t5892, t5895, t5896, t5899, t5902, t5907, t5908, t5911)
}
