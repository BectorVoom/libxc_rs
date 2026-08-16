//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta825 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2679;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2680;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta825(t1668: f64, t372: f64, t4823: f64, t1043: f64, t11249: f64, t11866: f64, t19976: f64, t19907: f64, t3241: f64, t1011: f64, t6288: f64, t697: f64, t11710: f64, t19872: f64, t3091: f64, t19968: f64, t3111: f64, t15850: f64, t4817: f64, t11921: f64, t19399: f64, t247: f64, t4837: f64, t15752: f64, t19741: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t66689, t66702, t66712, t66714, t66721) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2679(t1668, t372, t4823, t1043, t11249, t11866, t19976, t19907, t3241, t1011, t6288, t697);
        let (t66731, t66739, t66747, t66752, t66758) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2680(t11710, t19872, t3091, t19968, t3111, t15850, t4817, t11921, t19399, t247, t4837, t15752, t19741);
    (t66689, t66702, t66712, t66714, t66721, t66731, t66739, t66747, t66752, t66758)
}
