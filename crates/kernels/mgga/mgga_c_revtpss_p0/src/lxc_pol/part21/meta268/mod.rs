//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta268 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1483;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1484;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1485;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta268<F: Float>(t1390: F, t828: F, t9899: F, t221: F, t4019: F, t4057: F, t4018: F, t1386: F, t2681: F, t820: F, t1401: F, t4003: F, t9898: F, t4000: F, t843: F, t4006: F, t136: F, t4011: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t9901, t9905, t9906, t9909) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1483::<F>(t1390, t828, t9899, t221, t4019, t4057, t4018, t1386, t2681, t820);
        let (t9910, t9912) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1484::<F>(t1401, t9909, t4003, t9898);
        let (t9914, t9918, t9919, t9921) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1485::<F>(t1390, t828, t9912, t4000, t820, t843, t4006, t136, t4011);
    (t9901, t9905, t9906, t9909, t9910, t9912, t9914, t9918, t9919, t9921)
}
