//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 541/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk541(t3408: f64, t526: f64, t27: f64, t89: f64, t1957: f64, t1959: f64, t1962: f64, t3318: f64, t3321: f64, t3325: f64, t3328: f64, t3332: f64, t3335: f64, t3340: f64, t3345: f64) -> (f64, f64, f64) {
    let t3409 = t526 * t3408;
    let t3411 = t89 * t27 * t3409;
    let t3413 = t1957 + t1959 / 54.0_f64 + t1962 / 18.0_f64 + t3318 / 54.0_f64 - t3321 / 27.0_f64 + t3325 / 18.0_f64 + t3328 / 9.0_f64 - t3332 / 9.0_f64 + t3335 / 18.0_f64 + t3340 / 18.0_f64 + t3345 / 3.0_f64 - t3411 / 6.0_f64;
    (t3409, t3411, t3413)
}
