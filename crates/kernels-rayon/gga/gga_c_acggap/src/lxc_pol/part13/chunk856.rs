//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 856/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk856(t30077: f64, t177: f64, t2008: f64, t980: f64, t3646: f64, t588: f64, t2012: f64, t968: f64, t377: f64, t7370: f64, t2067: f64, t3077: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t30078 = 0.7558530601555998074e-1_f64 * t30077;
    let t30080 = t980 * t2008 * t177;
    let t30081 = 0.60023625365297631762e-2_f64 * t30080;
    let t30083 = t3646 * t588 * t177;
    let t30084 = 0.42874018118069736972e-3_f64 * t30083;
    let t30085 = t2012 * t968;
    let t30088 = t377 * t7370 * t177;
    let t30089 = 0.34013387707001991332e-1_f64 * t30088;
    let t30090 = t3077 * t2067;
    (t30078, t30081, t30084, t30085, t30089, t30090)
}
