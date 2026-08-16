//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1075/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1075(t102: f64, t108: f64, t176: f64, t203: f64, t23331: f64, t1909: f64, t188: f64, t1916: f64, t6756: f64, t1953: f64, t1972: f64, t201: f64, t2193: f64, t22285: f64, t22290: f64, t22293: f64, t23287: f64, t23289: f64, t23291: f64, t3308: f64, t5: f64, t6766: f64, t743: f64) -> f64 {
    let t23336 = t176 * t23331 * t102 * t108 * t203 / 2.0_f64;
    let t23341 = t1909 * t1909;
    let t23348 = t188 * t1916 * t6756;
    let t23350 = t1953 * t1953;
    let t23356 = t22285 + 2.0_f64 * t23287 + 140.0_f64 / 3.0_f64 * t23289 + 70.0_f64 / 3.0_f64 * t23291 - t22290 + t22293 + t23336 + 0.31013857721884116596e-1_f64 * t3308 * t1972 * t6766 * t2193 + t188 * t743 * t5 * t23341 * t201 / 2.0_f64 - 14.0_f64 / 3.0_f64 * t23348 + 3.0_f64 / 2.0_f64 * t188 * t743 * t5 * t23350 * t201;
    t23356
}
