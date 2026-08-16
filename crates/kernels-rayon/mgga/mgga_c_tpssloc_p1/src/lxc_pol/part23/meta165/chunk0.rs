//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 770/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk770(t9212: f64, t591: f64, t9: f64, t21: f64, t587: f64, t14: f64, t598: f64, t2230: f64, t594: f64, t2229: f64, t3: f64, t19: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9213 = 0.4332e2_f64 * t9212;
    let t9214 = t9 * t591;
    let t9215 = 0.9288e2_f64 * t9214;
    let t9216 = t587 * t21;
    let t9217 = 0.3912e3_f64 * t9216;
    let t9218 = t14 * t598;
    let t9219 = 0.12804e4_f64 * t9218;
    let t9220 = t594 * t2230;
    let t9221 = 0.170856e4_f64 * t9220;
    let t9222 = t2229 * t3;
    let t9223 = 1.0_f64 / t9222;
    let t9225 = 0.75936e3_f64 * t19 * t9223;
    (t9213, t9214, t9215, t9216, t9217, t9218, t9219, t9221, t9223, t9225)
}
