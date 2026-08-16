//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 670/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk670(t2096: f64, t7310: f64, t1133: f64, t570: f64, t2015: f64, t2028: f64, t2048: f64, t2016: f64, t2052: f64, t594: f64, t8: f64, t130: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7311 = t7310 * t2096;
    let t7312 = 0.6621875e-1_f64 * t7311;
    let t7313 = t570 * t1133;
    let t7315 = t2015 * t2028;
    let t7316 = t7315 * t2048;
    let t7317 = 11.0_f64 / 192.0_f64 * t7316;
    let t7318 = t2016 * t2052;
    let t7319 = 11.0_f64 / 576.0_f64 * t7318;
    let t7321 = 1.0_f64 / t8 / t594;
    let t7322 = t130 * t7321;
    (t7312, t7313, t7315, t7316, t7317, t7318, t7319, t7322)
}
