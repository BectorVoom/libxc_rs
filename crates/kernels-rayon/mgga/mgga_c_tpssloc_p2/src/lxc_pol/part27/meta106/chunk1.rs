//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 674/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk674(t681: f64, t125: f64, t701: f64, t141: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2409 = t681 * t681;
    let t2410 = 1.0_f64 / t2409;
    let t2411 = t125 * t2410;
    let t2412 = t701 * t701;
    let t2413 = t141 * t141;
    let t2414 = 1.0_f64 / t2413;
    let t2415 = t2412 * t2414;
    let t2417 = 0.16081979498692535067e2_f64 * t2411 * t2415;
    (t2409, t2410, t2411, t2412, t2413, t2414, t2415, t2417)
}
