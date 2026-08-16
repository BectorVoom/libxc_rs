//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 537/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk537(t28: f64, t3343: f64, t89: f64, t132: f64, t538: f64, t1009: f64, t1995: f64, t1008: f64, t549: f64, t554: f64, t2007: f64, t929: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3345 = t89 * t28 * t3343;
    let t3347 = t538 * t132;
    let t3348 = t3347 * t1009;
    let t3350 = t1995 * t1009;
    let t3355 = t549 * t1008;
    let t3356 = t3355 * t554;
    let t3359 = t2007 * t929;
    (t3345, t3347, t3348, t3350, t3355, t3356, t3359)
}
