//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 840/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk840(t3565: f64, t3578: f64, t144: f64, t4668: f64, t558: f64, t167: f64, t9432: f64, t609: f64, t2185: f64, t605: f64, t1017: f64, t3408: f64) -> (f64, f64, f64, f64, f64) {
    let t17062 = t3578 * t3565;
    let t17063 = t144 * t17062;
    let t17066 = t4668 * t558;
    let t17068 = t9432 * t167 * t17066;
    let t17071 = t4668 * t609;
    let t17073 = t2185 * t605 * t17071;
    let t17076 = t1017 * t3408;
    (t17062, t17063, t17068, t17073, t17076)
}
