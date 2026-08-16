//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 789/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk789(t30635: f64, t901: f64, t2389: f64, t9298: f64, t12448: f64, t2464: f64, t2487: f64, t29853: f64, t883: f64, t1538: f64, t9267: f64, t29984: f64, t4782: f64, t9272: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40023 = t30635 * t901;
    let t40073 = t9298 * t2389;
    let t40076 = t2487 * t2464 * t12448;
    let t40088 = t883 * t29853;
    let t40090 = t9267 * t1538 * t40088;
    let t40103 = t9272 * t4782 * t883 * t29984;
    (t40023, t40073, t40076, t40088, t40090, t40103)
}
