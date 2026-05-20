//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta364 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1729;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1730;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1731;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta364<F: Float>(t12003: F, t351: F, t3106: F, t3111: F, t3156: F, t3172: F, t3150: F, t11997: F, t3144: F, t3141: F, t11678: F, t4910: F, t3117: F, t1032: F, t3043: F, t1040: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t12004 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1729::<F>(t12003, t351);
        let (t12007, t12009, t12010, t12012, t12013, t12016, t12017, t12020) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1730::<F>(t3106, t3111, t3156, t3172, t3150, t11997, t3144, t3141, t11678, t4910, t3117, t1032, t3043);
        let t12021 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1731::<F>(t1040, t12020);
    (t12004, t12007, t12009, t12010, t12012, t12013, t12016, t12017, t12020, t12021)
}
