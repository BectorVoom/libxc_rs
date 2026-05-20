//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta261 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1459;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1460;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1461;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1462;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta261<F: Float>(t3869: F, t9866: F, t4010: F, t73: F, t1386: F, t2681: F, t820: F, t1401: F, t4000: F, t843: F, t136: F, t4011: F, t240: F, t532: F, t549: F, t72: F, t595: F, t66: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t9868, t9880, t9909) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1459::<F>(t3869, t9866, t4010, t73, t1386, t2681, t820);
        let (t9910, t9918, t9921) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1460::<F>(t1401, t9909, t4000, t820, t843, t136, t4011);
        let t9934 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1461::<F>(t240, t4000);
        let (t9940, t9941, t9942, t9948) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1462::<F>(t532, t549, t240, t72, t595, t66);
    (t9868, t9880, t9909, t9910, t9918, t9921, t9934, t9940, t9941, t9942, t9948)
}
