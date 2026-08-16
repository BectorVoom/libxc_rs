//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 788/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk788(t8796: f64, t1984: f64, t3408: f64, t558: f64, t28: f64, t89: f64, t3343: f64, t376: f64, t11402: f64, t3330: f64, t7773: f64, t998: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12346 = 4.0_f64 / 81.0_f64 * t8796;
    let t12350 = t1984 * t3408;
    let t12351 = t12350 * t558;
    let t12353 = t89 * t28 * t12351;
    let t12356 = t89 * t376 * t3343;
    let t12357 = 2.0_f64 / 9.0_f64 * t12356;
    let t12359 = t89 * t11402 * t3330;
    let t12362 = t89 * t7773 * t998;
    (t12346, t12353, t12356, t12357, t12359, t12362)
}
