//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1178/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1178(t28378: f64, t28405: f64, t235: f64, t5612: f64, t6657: f64, t5617: f64, t23008: f64, t5585: f64, t16758: f64, t232: f64, t6646: f64, t1888: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t28406 = t28378 + t28405;
    let t28407 = t235 * t28406;
    let t28409 = t6657 * t5612;
    let t28411 = t6657 * t5617;
    let t28413 = t23008 * t5585;
    let t28418 = t16758 * t232;
    let t28419 = t6646 * t28418;
    let t28420 = t1888 * t28419;
    (t28406, t28407, t28409, t28411, t28413, t28418, t28419, t28420)
}
