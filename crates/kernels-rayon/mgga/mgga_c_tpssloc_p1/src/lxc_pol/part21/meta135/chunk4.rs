//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 907/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk907(t1100: f64, t3279: f64, t3236: f64, t407: f64) -> (f64, f64, f64) {
    let t3280 = t1100 * t3279;
    let t3282 = 0.39862222222222222223e0_f64 * t3236;
    let t3287 = 1.0_f64/f64::sqrt(t407);
    (t3280, t3282, t3287)
}
