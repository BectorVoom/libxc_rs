//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 854/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk854(t2100: f64, t30044: f64, t7538: f64, t7544: f64, t1004: f64, t1979: f64, t7548: f64, t137: f64, t3101: f64, t1089: f64, t1095: f64, t2079: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t30045 = t30044 * t2100;
    let t30046 = 0.47172138434406228102e-3_f64 * t30045;
    let t30047 = t7538 * t7544;
    let t30048 = 0.47172138434406228102e-3_f64 * t30047;
    let t30049 = t1004 * t1979;
    let t30050 = t30049 * t7548;
    let t30051 = 0.62896184579208304135e-3_f64 * t30050;
    let t30052 = t137 * t3101;
    let t30055 = t2079 * t1089 * t1095 * t30052;
    (t30046, t30048, t30049, t30051, t30052, t30055)
}
