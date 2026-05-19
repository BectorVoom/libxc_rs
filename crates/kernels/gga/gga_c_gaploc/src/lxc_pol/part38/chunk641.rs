//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 641/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk641<F: Float>(t11259: F, t447: F, t1445: F, t11501: F, t11504: F, t11513: F, t11516: F, t11524: F, t11528: F, t1450: F, t1580: F, t193: F, t3557: F, t3561: F, t3566: F, t3592: F, t3596: F, t4511: F, t4953: F, t541: F, t574: F, t597: F, t9454: F) -> F {
    let t11531 = t11259 * t447;
    let t11532 = t1445 * t11531;
    let t11535 = -F::cast_from(0.30674340763136599741e1_f64) * t574 * t11501 - F::cast_from(0.61348681526273199483e1_f64) * t574 * t11504 + F::cast_from(0.23833659967900284446e0_f64) * t3561 * t541 - F::cast_from(0.7150097990370085334e0_f64) * t3566 * t9454 + F::cast_from(0.23833659967900284446e0_f64) * t3557 * t541 + F::cast_from(0.35750489951850426669e0_f64) * t11513 * t193 + F::cast_from(0.35750489951850426669e0_f64) * t11516 * t193 - F::cast_from(0.69017266717057349418e1_f64) * t4953 * t3592 + F::cast_from(0.23005755572352449806e1_f64) * t1580 * t3596 + F::cast_from(0.23005755572352449806e1_f64) * t597 * t11524 - F::cast_from(0.23005755572352449806e1_f64) * t1450 * t11528 + F::cast_from(0.69017266717057349418e1_f64) * t4511 * t11532;
    t11535
}
