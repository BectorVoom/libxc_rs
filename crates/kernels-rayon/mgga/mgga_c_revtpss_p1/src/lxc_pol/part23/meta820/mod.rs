//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta820 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2669;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2670;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta820(t15618: f64, t15682: f64, t1062: f64, t53877: f64, t15827: f64, t19878: f64, t15711: f64, t4834: f64, t11672: f64, t19785: f64, t1045: f64, t4772: f64, t15707: f64, t15769: f64, t12013: f64, t20029: f64, t1063: f64, t19671: f64, t3172: f64, t19697: f64, t3173: f64, t1041: f64, t19799: f64, t11262: f64, t6301: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t65823, t65837, t65840, t65859, t65892, t65894) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2669(t15618, t15682, t1062, t53877, t15827, t19878, t15711, t4834, t11672, t19785, t1045, t4772);
        let (t65931, t65960, t65965, t66003, t66017, t66022) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2670(t15707, t15769, t12013, t20029, t1063, t19671, t3172, t19697, t3173, t1041, t19799, t11262, t6301);
    (t65823, t65837, t65840, t65859, t65892, t65894, t65931, t65960, t65965, t66003, t66017, t66022)
}
