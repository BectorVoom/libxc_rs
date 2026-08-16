//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta287 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1067;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1068;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta287(t19826: f64, t3161: f64, t1058: f64, t6318: f64, t1062: f64, t15670: f64, t247: f64, t3109: f64, t6096: f64, t1063: f64, t140: f64, t6284: f64, t1011: f64, t6288: f64, t6292: f64, t3172: f64, t6262: f64, t3127: f64, t6317: f64, t11922: f64, t6272: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t19827, t19867, t19878, t19882, t19883, t19900) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1067(t19826, t3161, t1058, t6318, t1062, t15670, t247, t3109, t6096, t1063, t140, t6284);
        let (t19901, t19908, t19913, t19920, t19921, t19968, t19976) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1068(t1011, t19900, t140, t6288, t6292, t3172, t6262, t3127, t1062, t6317, t11922, t6272);
    (t19827, t19867, t19878, t19882, t19883, t19901, t19908, t19913, t19920, t19921, t19968, t19976)
}
