//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1227/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1227(t7609: f64, t9312: f64, t26514: f64, t898: f64, t26419: f64, t8522: f64, t2146: f64, t9274: f64, t9276: f64, t2165: f64, t26556: f64, t26634: f64, t2766: f64, t2771: f64, t2789: f64, t7660: f64, t7669: f64, t9010: f64, t9017: f64, t906: f64, t9185: f64, t92351: f64, t92356: f64) -> (f64, f64, f64, f64) {
    let t92360 = t7609 * t9312;
    let t92364 = t26514 * t898;
    let t92368 = 12.0_f64 * t8522 * t26419;
    let t92373 = t2146 * t9274;
    let t92375 = 6.0_f64 * t92373 * t9276;
    let t92376 = 2.0_f64 * t2165 * t2771 * t9185 + 6.0_f64 * t2771 * t2789 * t7669 - 18.0_f64 * t2789 * t7660 * t9017 - 3.0_f64 * t26556 * t2766 + 6.0_f64 * t26634 * t9010 - 3.0_f64 * t906 * t92364 + t92351 - t92356 + t92360 - t92368 + t92375;
    (t92360, t92368, t92375, t92376)
}
