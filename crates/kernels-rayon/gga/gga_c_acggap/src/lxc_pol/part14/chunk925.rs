//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 925/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk925(t1113: f64, t7736: f64, t377: f64, t7732: f64, t31404: f64, t7507: f64, t7517: f64, t3088: f64, t7646: f64, t3453: f64, t2138: f64, t2147: f64, t463: f64, t7993: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t31855 = t7736 * t1113;
    let t31863 = t377 * t7732;
    let t31867 = t7507 * t31404 * t7517;
    let t31868 = 0.1383716060742582691e-1_f64 * t31867;
    let t31878 = t3088 * t7646;
    let t31879 = t31878 * t3453;
    let t31905 = t2138 * t2147 * t7993 * t463;
    (t31855, t31863, t31868, t31878, t31879, t31905)
}
