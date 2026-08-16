//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta899 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3091;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3092;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta899(t15711: f64, t3106: f64, t15935: f64, t372: f64, t15904: f64, t245: f64, t3088: f64, t12167: f64, t1063: f64, t1592: f64, t247: f64, t42778: f64, t11922: f64, t16044: f64, t3115: f64, t11994: f64, t15769: f64, t3298: f64, t4746: f64, t4891: f64, t11744: f64, t4834: f64, t12009: f64, t15823: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t53724, t53728, t53739, t53740, t53741, t53762) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3091(t15711, t3106, t15935, t372, t15904, t245, t3088, t12167, t1063, t1592, t247, t42778);
        let (t53771, t53790, t53800, t53805, t53810) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3092(t11922, t16044, t3115, t11994, t15769, t3298, t4746, t4891, t11744, t4834, t12009, t15823);
    (t53724, t53728, t53739, t53740, t53741, t53762, t53771, t53790, t53800, t53805, t53810)
}
