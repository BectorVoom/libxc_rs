//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1590/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1590(t1145: f64, t141: f64, t43847: f64, t12283: f64, t698: f64, t43858: f64, t43862: f64, t43865: f64, t43871: f64, t43877: f64, t43883: f64, t43909: f64, t43911: f64, t43914: f64, t43917: f64, t43920: f64, t43923: f64) -> (f64, f64, f64) {
    let t43926 = t141 * t1145 * t43847;
    let t43928 = t698 * t12283;
    let t43936 = 0.258925e1_f64 * t43909 - 0.18396666666666666667e0_f64 * t43911 - 0.82785e-1_f64 * t43914 + 0.49671e0_f64 * t43917 - 0.11038e0_f64 * t43920 - 0.99342e0_f64 * t43923 + 0.66228e0_f64 * t43926 + 0.22076e0_f64 * t43928 - 0.44729629629629629629e0_f64 * t43858 - 0.89459259259259259259e0_f64 * t43862 - 0.53675555555555555556e0_f64 * t43865 - 0.60384999999999999999e0_f64 * t43871 + 0.181155e1_f64 * t43877 + 0.16102666666666666667e1_f64 * t43883;
    (t43926, t43928, t43936)
}
