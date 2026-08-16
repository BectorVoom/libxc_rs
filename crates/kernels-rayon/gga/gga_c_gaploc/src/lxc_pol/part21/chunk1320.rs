//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1320/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1320(t34502: f64, t1: f64, t106: f64, t4524: f64, t544: f64, t191: f64, t4529: f64, t34378: f64, t10517: f64, t7014: f64, t10615: f64, t31167: f64) -> (f64, f64, f64, f64) {
    let t34503 = 0.89376224879626066674e-1_f64 * t34502;
    let t34506 = t544 * t4524 * t1 * t106;
    let t34507 = t191 * t4529;
    let t34510 = 0.85801175884441024004e1_f64 * t34506 * t34507 * t34378;
    let t34512 = 0.87421871174939309262e2_f64 * t7014 * t10517;
    let t34530 = t10615 * t31167;
    (t34503, t34510, t34512, t34530)
}
