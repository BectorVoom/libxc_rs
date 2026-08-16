//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 795/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk795(t7524: f64, t7525: f64, t7527: f64, t7529: f64, t7531: f64, t7535: f64, t7538: f64, t7541: f64, t7544: f64, t7547: f64, t7550: f64, t787: f64) -> (f64, f64) {
    let t7552 = -t7524 - 4.0_f64 / 9.0_f64 * t7525 + 2.0_f64 / 9.0_f64 * t7527 - 2.0_f64 / 3.0_f64 * t7529 + t7531 / 3.0_f64 - 10.0_f64 / 27.0_f64 * t7535 + 4.0_f64 / 3.0_f64 * t7538 - 2.0_f64 / 3.0_f64 * t7541 - 2.0_f64 * t7544 + 2.0_f64 * t7547 - t7550 / 3.0_f64;
    let t7553 = t787 * t7552;
    (t7552, t7553)
}
