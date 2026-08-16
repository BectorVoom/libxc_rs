//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1415/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1415(t1985: f64, t214: f64, t225: f64, t27051: f64, t567: f64, t22666: f64, t33296: f64, t7918: f64, t6907: f64, t33259: f64, t115354: f64, t120232: f64, t120239: f64, t120244: f64, t120247: f64, t120253: f64, t120258: f64, t1386: f64, t2016: f64, t2092: f64, t26366: f64, t7214: f64, t91441: f64, t93316: f64) -> (f64, f64) {
    let t122160 = t1985 * t214 * t27051 * t225 * t567;
    let t122164 = t1985 * t22666 * t33296;
    let t122166 = t214 * t7918;
    let t122168 = t1985 * t122166 * t6907;
    let t122172 = t33259 * t225;
    let t122174 = 0.82246703342411321824e-2_f64 * t115354 + 0.82246703342411321825e-2_f64 * t122160 - t120232 - t91441 * t2092 - 0.82246703342411321825e-2_f64 * t122164 - 0.82246703342411321825e-2_f64 * t122168 - t120239 - t120244 - t26366 * t7214 + t120247 + t120253 - t93316 * t2016 - t122172 * t1386 + t120258;
    (t122166, t122174)
}
