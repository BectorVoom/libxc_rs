//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1386/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1386(t1655: f64, t26654: f64, t1657: f64, t18402: f64, t2169: f64, t2209: f64, t233: f64, t27155: f64, t27734: f64, t27746: f64, t2802: f64, t3703: f64, t7673: f64, t8024: f64, t8121: f64, t92344: f64, t92351: f64, t92356: f64, t92360: f64, t92368: f64, t92375: f64) -> f64 {
    let t97601 = t1655 * t26654;
    let t97602 = t92344 - t27155 * t8024 / 8.0_f64 - t233 * t2802 * t8121 / 16.0_f64 - t92351 - t233 * t18402 * t2209 / 16.0_f64 + t92356 - t92360 + t7673 * t27734 / 8.0_f64 + t92368 - t92375 - t2169 * t1657 * t3703 / 16.0_f64 + t7673 * t27746 / 8.0_f64 + t97601;
    t97602
}
