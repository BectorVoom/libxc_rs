//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 789/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk789(t3409: f64, t375: f64, t89: f64, t3379: f64, t549: f64, t554: f64, t2071: f64, t3355: f64, t135: f64, t3347: f64, t538: f64, t1995: f64, t3380: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12365 = t89 * t375 * t3409;
    let t12366 = t12365 / 9.0_f64;
    let t12367 = t549 * t3379;
    let t12368 = t12367 * t554;
    let t12371 = t3355 * t2071;
    let t12374 = t3347 * t135;
    let t12381 = t538 * t554;
    let t12385 = t1995 * t3380;
    (t12365, t12366, t12368, t12371, t12374, t12381, t12385)
}
