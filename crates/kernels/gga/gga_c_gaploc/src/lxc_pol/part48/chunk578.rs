//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 578/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk578<F: Float>(t1628: F, t3585: F, t3576: F, t3556: F, t524: F, t3560: F, t11218: F, t600: F, t568: F, t11254: F, t447: F, t1445: F, t11259: F, t1450: F, t1580: F, t193: F, t3557: F, t3561: F, t3566: F, t3592: F, t3596: F, t4511: F, t4953: F, t541: F, t574: F, t597: F, t9454: F) -> (F,) {
    let t11501 = t1628 * t3585;
    let t11504 = t1628 * t3576;
    let t11513 = t524 * t3556;
    let t11516 = t524 * t3560;
    let t11523 = t600 * t11218;
    let t11524 = t568 * t11523;
    let t11527 = t11254 * t447;
    let t11528 = t1445 * t11527;
    let t11531 = t11259 * t447;
    let t11532 = t1445 * t11531;
    let t11535 = -0.30674340763136599741e1 * t574 * t11501 - 0.61348681526273199483e1 * t574 * t11504 + 0.23833659967900284446e0 * t3561 * t541 - 0.7150097990370085334e0 * t3566 * t9454 + 0.23833659967900284446e0 * t3557 * t541 + 0.35750489951850426669e0 * t11513 * t193 + 0.35750489951850426669e0 * t11516 * t193 - 0.69017266717057349418e1 * t4953 * t3592 + 0.23005755572352449806e1 * t1580 * t3596 + 0.23005755572352449806e1 * t597 * t11524 - 0.23005755572352449806e1 * t1450 * t11528 + 0.69017266717057349418e1 * t4511 * t11532;
    (t11535,)
}
