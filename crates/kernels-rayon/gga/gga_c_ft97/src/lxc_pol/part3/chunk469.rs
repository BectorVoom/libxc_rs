//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 469/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk469(t3140: f64, t582: f64, t1037: f64, t458: f64, t2102: f64, t3338: f64, t1017: f64, t2: f64, t1985: f64, t558: f64, t24: f64, t3408: f64, t586: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3510 = t582 * t3140;
    let t3513 = t458 * t1037;
    let t3515 = t2102 * t3338;
    let t3518 = t2 * t1017;
    let t3520 = t1985 * t3518 * t558;
    let t3524 = t24 * t586 * t3408;
    (t3510, t3513, t3515, t3518, t3520, t3524)
}
