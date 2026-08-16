//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 713/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk713(t1772: f64, t312: f64, t310: f64, t307: f64, t7253: f64, t7256: f64, t906: f64, t317: f64, t1: f64, t2672: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7894 = t1772 * t312;
    let t7895 = t310 * t7894;
    let t7897 = 0.80492236016562572729e-3_f64 * t307 * t7895;
    let t7924 = t7253 * t7256;
    let t7946 = t906 * t906;
    let t7947 = 1.0_f64 / t7946;
    let t7948 = t317 * t7947;
    let t8002 = t2672 * t1;
    (t7894, t7895, t7897, t7924, t7946, t7947, t7948, t8002)
}
