//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 868/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk868(t2347: f64, t984: f64, t2351: f64, t988: f64, t355: f64, t7592: f64, t7529: f64, t7538: f64, t7541: f64, t7544: f64, t7547: f64, t7560: f64, t7563: f64, t7566: f64, t7596: f64, t7599: f64) -> (f64, f64, f64, f64) {
    let t8304 = t984 * t2347;
    let t8306 = t988 * t2351;
    let t8307 = t355 * t8306;
    let t8319 = 0.54733333333333333333e-2_f64 * t7592;
    let t8320 = -0.4926e-2_f64 * t7560 + 0.2463e-2_f64 * t7563 - 0.12315e-2_f64 * t7596 - 0.7389e-2_f64 * t7566 + 0.7389e-2_f64 * t7599 - 0.38776666666666666665e1_f64 * t7529 + 0.77553333333333333331e1_f64 * t7538 - 0.38776666666666666665e1_f64 * t7541 - 0.11633e2_f64 * t7544 + 0.11633e2_f64 * t7547 - t8319;
    (t8304, t8306, t8307, t8320)
}
