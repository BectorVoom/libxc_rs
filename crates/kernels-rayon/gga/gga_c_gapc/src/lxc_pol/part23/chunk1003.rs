//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 1003/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk1003(t6: f64, t7592: f64, t7593: f64, t2776: f64, t286: f64, t8132: f64, t7591: f64, t8141: f64, t952: f64, t291: f64, t4043: f64, t959: f64) -> (f64, f64, f64, f64, f64) {
    let t16152 = t7592 * t7593 * t6;
    let t16181 = t2776 * t286;
    let t16182 = t8132 * t16181;
    let t16296 = t7591 * t952 * t8141;
    let t16403 = t4043 * t291 * t959;
    (t16152, t16181, t16182, t16296, t16403)
}
