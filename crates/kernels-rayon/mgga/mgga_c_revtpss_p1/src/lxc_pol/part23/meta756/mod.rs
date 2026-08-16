//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta756 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2546;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2547;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta756(t15711: f64, t3106: f64, t15935: f64, t372: f64, t15904: f64, t245: f64, t3088: f64, t12167: f64, t1063: f64, t1592: f64, t247: f64, t42778: f64, t3298: f64, t4746: f64, t4891: f64, t12012: f64, t15822: f64, t1086: f64, t15654: f64, t3090: f64, t1025: f64, t371: f64, t4852: f64, t676: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t53724, t53728, t53739, t53740, t53741, t53762) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2546(t15711, t3106, t15935, t372, t15904, t245, t3088, t12167, t1063, t1592, t247, t42778);
        let (t53800, t53807, t53855, t53875) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2547(t3298, t4746, t4891, t12012, t15822, t1086, t15654, t3090, t1025, t371, t4852, t676);
    (t53724, t53728, t53739, t53740, t53741, t53762, t53800, t53807, t53855, t53875)
}
