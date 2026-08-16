//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1900/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1900(t109: f64, t1873: f64, t28002: f64, t4028: f64, t7467: f64, t5493: f64, t88: f64, t7676: f64, t22473: f64, t5464: f64, t5488: f64, t6530: f64, t22469: f64, t27166: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t110 = 1.0_f64 < t109;
    let t28004 = 4.0_f64 * t28002 * t1873;
    let t28006 = 4.0_f64 * t4028 * t7467;
    let t28007 = t88 * t5493;
    let t28009 = 2.0_f64 * t28007 * t1873;
    let t28011 = 4.0_f64 * t7676 * t7467;
    let t28012 = t22473 * t5464;
    let t28014 = t6530 * t5488;
    let t28017 = piecewise3(t110, 0.0_f64, t22469 + t27166 + t28012 / 4.0_f64 - t28014 / 8.0_f64);
    (t28004, t28006, t28007, t28009, t28011, t28017)
}
