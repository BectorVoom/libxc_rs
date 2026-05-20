//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta903 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3099;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3100;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta903<F: Float>(t16166: F, t3127: F, t3172: F, t16171: F, t42793: F, t4899: F, t4901: F, t11710: F, t16095: F, t16097: F, t16127: F, t43131: F, t16088: F, t3046: F, t380: F, t16139: F, t1011: F, t1655: F, t2438: F, t1014: F, t4579: F, t697: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t54042, t54047, t54078, t54081, t54085) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3099::<F>(t16166, t3127, t3172, t16171, t42793, t4899, t4901, t11710, t16095, t16097, t16127, t43131);
        let (t54089, t54099, t54118, t54122) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3100::<F>(t16088, t3046, t380, t16139, t3127, t3172, t1011, t1655, t2438, t1014, t4579, t697);
    (t54042, t54047, t54078, t54081, t54085, t54089, t54099, t54118, t54122)
}
