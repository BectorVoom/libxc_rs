//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 685/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk685(t1736: f64, t2248: f64, t422: f64, t1725: f64, t3085: f64, t626: f64, t934: f64, t419: f64, t3095: f64, t8715: f64, t122: f64, t409: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11273 = t2248 * t1736;
    let t11280 = t2248 * t422;
    let t11296 = t1725 * t3085;
    let t11297 = 0.1134997482304526749e-1_f64 * t11296;
    let t11298 = t626 * t934;
    let t11299 = t419 * t11298;
    let t11303 = t8715 * t3095;
    let t11304 = t419 * t11303;
    let t11360 = t409 * t122;
    (t11273, t11280, t11296, t11297, t11299, t11304, t11360)
}
