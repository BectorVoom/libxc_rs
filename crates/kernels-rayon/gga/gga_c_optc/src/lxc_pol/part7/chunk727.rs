//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 727/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk727(t1928: f64, t2035: f64, t6931: f64, t127: f64, t2022: f64, t616: f64, t2034: f64, t2010: f64, t623: f64, t2013: f64, t56: f64, t658: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6932 = t2035 * t1928;
    let t6933 = t6931 * t6932;
    let t6936 = t2022 * t127;
    let t6937 = t6936 * t616;
    let t6938 = t2034 * t6937;
    let t6941 = t623 * t2010;
    let t6942 = t6941 * t2013;
    let t6944 = t56 * t658;
    (t6932, t6933, t6936, t6937, t6938, t6941, t6942, t6944)
}
