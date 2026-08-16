//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta911 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3115;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3116;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta911(t11223: f64, t16088: f64, t380: f64, t1041: f64, t16185: f64, t3172: f64, t1062: f64, t42261: f64, t11710: f64, t15974: f64, t4899: f64, t11866: f64, t15794: f64, t11671: f64, t15925: f64, t15752: f64, t15917: f64, t127: f64, t15700: f64, t15702: f64, t4801: f64, t1063: f64, t11986: f64, t247: f64, t4583: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t54857, t54869, t54899, t54907, t54914) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3115(t11223, t16088, t380, t1041, t16185, t3172, t1062, t42261, t11710, t15974, t4899, t11866, t15794);
        let (t54916, t54919, t54925, t54943) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3116(t11671, t15925, t15752, t15917, t127, t15700, t15702, t4801, t1063, t11986, t247, t4583);
    (t54857, t54869, t54899, t54907, t54914, t54916, t54919, t54925, t54943)
}
