//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 627/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk627(t1212: f64, t1248: f64, t840: f64, t871: f64, t1234: f64, t2755: f64, t91: f64, t2766: f64, t5098: f64, t2771: f64, t5213: f64, t5105: f64, t848: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5330 = t1212 * t1248;
    let t5332 = t840 * t871 * t5330;
    let t5337 = t1234 * t1234;
    let t5339 = t91 * t2755 * t5337;
    let t5343 = t2766 * t5098;
    let t5346 = t2771 * t5213;
    let t5349 = t848 * t5105;
    (t5330, t5332, t5337, t5339, t5343, t5346, t5349)
}
