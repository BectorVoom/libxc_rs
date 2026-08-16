//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta762 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2558;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta762(t342: f64, t378: f64, t43536: f64, t11631: f64, t43350: f64, t43346: f64, t42872: f64, t12046: f64, t1647: f64, t12153: f64, t4746: f64, t15654: f64, t3286: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t55569, t55570, t55593, t55594, t55599, t55646, t55685) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2558(t342, t378, t43536, t11631, t43350, t43346, t42872, t12046, t1647, t12153, t4746, t15654, t3286);
    (t55569, t55570, t55593, t55594, t55599, t55646, t55685)
}
