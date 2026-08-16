//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 984/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk984(t10415: f64, t330: f64, t1098: f64, t3305: f64, t1111: f64, t3251: f64, t1116: f64, t3300: f64, t1088: f64, t3245: f64, t1014: f64, t3171: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10416 = t10415 * t330;
    let t10422 = t1098 * t3305;
    let t10424 = t3251 * t1111;
    let t10426 = t3251 * t1116;
    let t10428 = t1098 * t3300;
    let t10450 = t3245 * t1088;
    let t10452 = t1014 * t3171;
    (t10416, t10422, t10424, t10426, t10428, t10450, t10452)
}
