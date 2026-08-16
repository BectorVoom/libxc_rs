//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 641/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk641(t11259: f64, t447: f64, t1445: f64, t11501: f64, t11504: f64, t11513: f64, t11516: f64, t11524: f64, t11528: f64, t1450: f64, t1580: f64, t193: f64, t3557: f64, t3561: f64, t3566: f64, t3592: f64, t3596: f64, t4511: f64, t4953: f64, t541: f64, t574: f64, t597: f64, t9454: f64) -> f64 {
    let t11531 = t11259 * t447;
    let t11532 = t1445 * t11531;
    let t11535 = -0.30674340763136599741e1_f64 * t574 * t11501 - 0.61348681526273199483e1_f64 * t574 * t11504 + 0.23833659967900284446e0_f64 * t3561 * t541 - 0.7150097990370085334e0_f64 * t3566 * t9454 + 0.23833659967900284446e0_f64 * t3557 * t541 + 0.35750489951850426669e0_f64 * t11513 * t193 + 0.35750489951850426669e0_f64 * t11516 * t193 - 0.69017266717057349418e1_f64 * t4953 * t3592 + 0.23005755572352449806e1_f64 * t1580 * t3596 + 0.23005755572352449806e1_f64 * t597 * t11524 - 0.23005755572352449806e1_f64 * t1450 * t11528 + 0.69017266717057349418e1_f64 * t4511 * t11532;
    t11535
}
