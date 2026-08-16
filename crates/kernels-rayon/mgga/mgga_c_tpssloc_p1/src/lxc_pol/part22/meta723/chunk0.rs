//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2368/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2368(t21160: f64, t699: f64, t21167: f64, t47705: f64, t47707: f64, t48103: f64, t49139: f64, t49144: f64, t68442: f64, t68444: f64, t68446: f64, t68448: f64) -> (f64, f64, f64) {
    let t68452 = t699 * t21160;
    let t68454 = t699 * t21167;
    let t68457 = 0.60385e0_f64 * t68442 + 0.10064166666666666667e0_f64 * t68444 + 0.11182407407407407407e0_f64 * t68446 - 0.40256666666666666667e0_f64 * t68448 + 0.80513333333333333336e0_f64 * t47705 - 0.26837777777777777779e0_f64 * t47707 - t49139 - t49144 - 0.33114e0_f64 * t68452 + 0.5519e-1_f64 * t68454 + 0.73586666666666666667e0_f64 * t48103;
    (t68452, t68454, t68457)
}
