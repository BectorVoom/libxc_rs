//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2798/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2798(t1573: f64, t40317: f64, t39692: f64, t39694: f64, t39697: f64, t39701: f64, t39707: f64, t4514: f64, t51380: f64, t51435: f64, t51438: f64, t51442: f64, t51445: f64, t837: f64) -> f64 {
    let t51452 = t40317 * t1573;
    let t51456 = t51435 + 0.32927245914677557992e-1_f64 * t51438 - 0.29272321618148349057e-1_f64 * t51442 + 0.30356481678079769392e-1_f64 * t51445 - 0.29272321618148349057e-1_f64 * t39692 + 0.19514881078765566037e-2_f64 * t39694 + t39697 - 0.19756347548806534796e1_f64 * t4514 * t51380 * t837 + 0.11044544084478153697e-3_f64 * t51452 - 0.58911598146606471822e-3_f64 * t39701 + 0.16463622957338778996e-1_f64 * t39707;
    t51456
}
