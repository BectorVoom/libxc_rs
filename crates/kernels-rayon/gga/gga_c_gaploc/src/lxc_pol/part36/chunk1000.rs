//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 1000/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk1000(t43912: f64, t32744: f64, t9824: f64, t10924: f64, t1980: f64, t13065: f64, t2013: f64, t43710: f64, t825: f64, t969: f64, t41342: f64, t13072: f64, t32969: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t43913 = 0.59584149919750711116e-1_f64 * t43912;
    let t43914 = t32744 * t9824;
    let t43915 = 0.29792074959875355558e-1_f64 * t43914;
    let t43917 = t1980 * t10924 * t9824;
    let t43918 = 0.29792074959875355558e-1_f64 * t43917;
    let t43919 = t2013 * t13065;
    let t43922 = t825 * t969 * t43710;
    let t43924 = 0.29792074959875355558e-1_f64 * t41342;
    let t43925 = t32969 * t13072;
    (t43913, t43915, t43918, t43919, t43922, t43924, t43925)
}
