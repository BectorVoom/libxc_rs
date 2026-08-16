//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1397/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1397(t1442: f64, t52330: f64, t52331: f64, t19: f64, t5328: f64, t8974: f64, t4356: f64, t3119: f64, t55127: f64, t5311: f64, t3104: f64, t438: f64, t935: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t58917 = t52330 * t52331 * t1442;
    let t58922 = t5328 * t19;
    let t58923 = t58922 * t8974;
    let t58928 = t58922 * t4356;
    let t58932 = t55127 * t3119;
    let t58941 = t5311 * t5311;
    let t58942 = t3104 * t58941;
    let t58947 = t5328 * t935 * t438;
    (t58917, t58923, t58928, t58932, t58941, t58942, t58947)
}
