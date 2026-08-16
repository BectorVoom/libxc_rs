//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta903 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3099;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3100;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta903(t16166: f64, t3127: f64, t3172: f64, t16171: f64, t42793: f64, t4899: f64, t4901: f64, t11710: f64, t16095: f64, t16097: f64, t16127: f64, t43131: f64, t16088: f64, t3046: f64, t380: f64, t16139: f64, t1011: f64, t1655: f64, t2438: f64, t1014: f64, t4579: f64, t697: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t54042, t54047, t54078, t54081, t54085) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3099(t16166, t3127, t3172, t16171, t42793, t4899, t4901, t11710, t16095, t16097, t16127, t43131);
        let (t54089, t54099, t54118, t54122) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3100(t16088, t3046, t380, t16139, t3127, t3172, t1011, t1655, t2438, t1014, t4579, t697);
    (t54042, t54047, t54078, t54081, t54085, t54089, t54099, t54118, t54122)
}
