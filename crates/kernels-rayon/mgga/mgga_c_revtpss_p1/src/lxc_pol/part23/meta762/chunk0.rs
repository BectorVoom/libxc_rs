//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2558/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2558(t342: f64, t378: f64, t43536: f64, t11631: f64, t43350: f64, t43346: f64, t42872: f64, t12046: f64, t1647: f64, t12153: f64, t4746: f64, t15654: f64, t3286: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t55569 = t342 * t43536 * t378;
    let t55570 = t43350 * t11631;
    let t55593 = t342 * t43346 * t378;
    let t55594 = t43350 * t42872;
    let t55599 = t1647 * t12046;
    let t55646 = t4746 * t12153;
    let t55685 = t15654 * t3286;
    (t55569, t55570, t55593, t55594, t55599, t55646, t55685)
}
