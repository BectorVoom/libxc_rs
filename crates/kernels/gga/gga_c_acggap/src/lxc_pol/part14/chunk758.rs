//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 758/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk758<F: Float>(t1083: F, t1089: F, t9563: F, t598: F, t1861: F, t2001: F, t1851: F, t3300: F, t9552: F, t1095: F, t4352: F, t9529: F, t7328: F, t7373: F, t7376: F, t9214: F, t9215: F, t9222: F, t9532: F, t9534: F, t9539: F, t9544: F, t9546: F, t9550: F, t9555: F, t9557: F, t9561: F) -> (F, F, F, F) {
    let t9565 = t1089 * t1083 * t9563;
    let t9566 = t598 * t9565;
    let t9568 = t2001 * t1861;
    let t9570 = t2001 * t1851;
    let t9573 = t1089 * t3300 * t9552;
    let t9574 = t598 * t9573;
    let t9577 = t4352 * t1095 * t9529;
    let t9578 = t598 * t9577;
    let t9580 = -t9214 - t9215 + 0.32155513588552302729e-2 * t9532 - 0.42874018118069736972e-3 * t9534 + 0.7862023072401038017e-3 * t9539 - 0.31448092289604152068e-3 * t9544 - 0.42874018118069736972e-3 * t9546 - 0.10718504529517434243e-2 * t9550 - 0.18868855373762491241e-2 * t9555 + 0.85748036236139473944e-3 * t9557 - 0.42874018118069736972e-3 * t9561 - 0.21437009059034868486e-3 * t9566 - 0.68598428988911579156e-2 * t9568 + 0.68598428988911579156e-2 * t9570 - t7328 + t9222 + 0.64311027177104605458e-3 * t9574 - 0.47172138434406228102e-2 * t9578 + t7373 - t7376;
    (t9565, t9573, t9577, t9580)
}
