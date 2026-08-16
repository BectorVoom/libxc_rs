//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 242/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk242(t10: f64, t1542: f64, t296: f64, t2336: f64, t793: f64, t89: f64, t375: f64, t825: f64, t683: f64, t798: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2652 = t10 * t1542 * t296;
    let t2653 = 2.0_f64 / 27.0_f64 * t2652;
    let t2655 = t89 * t2336 * t793;
    let t2656 = t2655 / 27.0_f64;
    let t2658 = t89 * t375 * t825;
    let t2659 = t2658 / 9.0_f64;
    let t2665 = t683 * t798;
    (t2652, t2653, t2655, t2656, t2658, t2659, t2665)
}
