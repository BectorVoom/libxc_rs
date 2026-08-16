//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2559/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2559(t16543: f64, t3046: f64, t4746: f64, t4995: f64, t15669: f64, t3286: f64, t1651: f64, t378: f64, t342: f64, t43400: f64, t3057: f64, t12077: f64, t1647: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t55701 = t3046 * t16543;
    let t55732 = t4746 * t4995;
    let t55747 = t15669 * t3286;
    let t55764 = t378 * t1651;
    let t55805 = t342 * t43400 * t378;
    let t55887 = t3057 * t16543;
    let t55899 = t1647 * t12077;
    (t55701, t55732, t55747, t55764, t55805, t55887, t55899)
}
