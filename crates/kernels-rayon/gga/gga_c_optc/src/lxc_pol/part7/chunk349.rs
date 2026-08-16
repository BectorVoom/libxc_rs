//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 349/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk349(t1128: f64, t310: f64, t448: f64, t309: f64, t447: f64, t441: f64) -> (f64, f64, f64, f64) {
    let t1129 = t310 * t1128;
    let t1131 = 0.18110753103726578864e-2_f64 * t448 * t1129;
    let t1132 = t447 * t309;
    let t1133 = t441 * t1132;
    (t1129, t1131, t1132, t1133)
}
