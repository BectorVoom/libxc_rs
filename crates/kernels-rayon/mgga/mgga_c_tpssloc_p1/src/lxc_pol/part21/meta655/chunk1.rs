//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2454/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2454(t43688: f64, t3402: f64, t3639: f64, t2394: f64, t3244: f64) -> (f64, f64, f64, f64) {
    let t43689 = 1.0_f64 / t43688;
    let t43691 = t3402 * t3402;
    let t43692 = 1.0_f64 / t43691;
    let t43705 = t3639 * t3639;
    let t43706 = 1.0_f64 / t43705;
    let t43748 = t2394 * t3244;
    (t43689, t43692, t43706, t43748)
}
