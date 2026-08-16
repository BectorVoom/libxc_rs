//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 976/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk976(t28: f64, t28447: f64, t1649: f64, t7540: f64, t126197: f64, t25927: f64, t19451: f64, t8327: f64, t28002: f64, t32677: f64, t4028: f64, t28237: f64, t3701: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t126992 = t28 * t28447;
    let t127017 = t1649 * t7540;
    let t127030 = t25927 * t126197;
    let t127107 = 2.0_f64 * t19451 * t8327;
    let t127109 = 4.0_f64 * t28002 * t8327;
    let t127111 = 4.0_f64 * t4028 * t32677;
    let t127114 = t3701 * t28237;
    (t126992, t127017, t127030, t127107, t127109, t127111, t127114)
}
