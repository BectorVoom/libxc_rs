//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta760 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2554;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2555;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta760(t15731: f64, t3169: f64, t12078: f64, t53740: f64, t12047: f64, t16138: f64, t372: f64, t11671: f64, t15925: f64, t1063: f64, t11986: f64, t247: f64, t4583: f64, t1062: f64, t43154: f64, t16088: f64, t342: f64, t380: f64, t16219: f64, t3241: f64, t11262: f64, t4802: f64, t4807: f64, t11773: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t54733, t54801, t54811, t54818, t54916, t54943) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2554(t15731, t3169, t12078, t53740, t12047, t16138, t372, t11671, t15925, t1063, t11986, t247, t4583);
        let (t54982, t55011, t55034, t55062, t55065, t55141) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2555(t1062, t43154, t16088, t342, t380, t16219, t3241, t1063, t11262, t4802, t4807, t11773, t15925);
    (t54733, t54801, t54811, t54818, t54916, t54943, t54982, t55011, t55034, t55062, t55065, t55141)
}
