//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta490 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2213;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2214;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta490(t16152: f64, t247: f64, t3116: f64, t3109: f64, t4583: f64, t1063: f64, t3172: f64, t4868: f64, t1041: f64, t2862: f64, t4823: f64, t1042: f64, t1651: f64, t3181: f64, t2853: f64, t15100: f64, t15103: f64, t15377: f64, t15379: f64, t15382: f64, t15385: f64, t15388: f64, t15392: f64, t15395: f64, t15519: f64, t15522: f64, t15524: f64, t15528: f64, t15530: f64, t15536: f64, t15540: f64, t15545: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16154, t16158, t16160, t16163, t16165, t16166, t16167) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2213(t16152, t247, t3116, t3109, t4583, t1063, t3172, t4868, t1041, t2862, t4823, t1042);
        let (t16170, t16171, t16172, t16179) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2214(t1651, t3181, t2853, t1042, t15100, t15103, t15377, t15379, t15382, t15385, t15388, t15392, t15395, t15519, t15522, t15524, t15528, t15530, t15536, t15540, t15545);
    (t16154, t16158, t16160, t16163, t16165, t16166, t16167, t16170, t16171, t16172, t16179)
}
