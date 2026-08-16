//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta290 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1157;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1158;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta290(t1089: f64, t11928: f64, t1071: f64, t1086: f64, t994: f64, t11869: f64, t3316: f64, t989: f64, t1082: f64, t11804: f64, t11239: f64, t11627: f64, t342: f64, t11631: f64, t12051: f64, t12048: f64, t1024: f64, t1083: f64, t1087: f64, t11782: f64, t12111: f64, t12116: f64, t12119: f64, t12122: f64, t12124: f64, t12127: f64, t12128: f64, t12133: f64, t12137: f64, t12143: f64, t12146: f64, t12149: f64, t3204: f64, t3223: f64, t3287: f64, t3288: f64, t3292: f64, t3295: f64, t3305: f64, t3319: f64, t4981: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12150, t12153, t12154, t12157, t12160, t12163, t12166) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1157(t1089, t11928, t1071, t1086, t994, t11869, t3316, t989, t1082, t11804, t11239, t11627);
        let (t12167, t12168, t12169, t12172) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1158(t12166, t342, t11631, t12051, t12048, t1024, t1083, t1087, t11782, t12111, t12116, t12119, t12122, t12124, t12127, t12128, t12133, t12137, t12143, t12146, t12149, t12150, t12154, t12157, t12160, t12163, t3204, t3223, t3287, t3288, t3292, t3295, t3305, t3319, t4981);
    (t12150, t12153, t12154, t12157, t12160, t12163, t12166, t12167, t12168, t12169, t12172)
}
