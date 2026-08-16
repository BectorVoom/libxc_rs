//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2681/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2681(t3091: f64, t43240: f64, t6267: f64, t16088: f64, t380: f64, t4746: f64, t1065: f64, t372: f64, t6299: f64, t3105: f64, t6317: f64, t15794: f64, t15926: f64) -> (f64, f64, f64, f64, f64) {
    let t66763 = t3091 * t43240 * t6267;
    let t66766 = t4746 * t380 * t16088;
    let t66777 = t372 * t1065 * t6299;
    let t66784 = t6317 * t3105;
    let t66814 = t15926 * t15794;
    (t66763, t66766, t66777, t66784, t66814)
}
