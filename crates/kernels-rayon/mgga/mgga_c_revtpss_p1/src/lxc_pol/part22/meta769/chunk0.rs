//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2854/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2854(t2439: f64, t3418: f64, t406: f64, t12555: f64, t3515: f64, t43813: f64, t1126: f64, t12226: f64, t3382: f64, t3431: f64, t408: f64, t43816: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t43911 = t2439 * t3418;
    let t43946 = f64::powf(t406, -0.25e1_f64);
    let t43977 = t12555 * t3515;
    let t43995 = 0.96141975308641975307e-1_f64 * t43813;
    let t44012 = t1126 * t12226;
    let t44017 = t408 / t3431 / t3382;
    let t44039 = 0.31003950617283950618e1_f64 * t43813;
    let t44040 = 0.13388493827160493828e1_f64 * t43816;
    (t43911, t43946, t43977, t43995, t44012, t44017, t44039, t44040)
}
