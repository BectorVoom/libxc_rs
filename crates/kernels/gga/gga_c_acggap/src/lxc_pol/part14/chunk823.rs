//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 823/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk823<F: Float>(t598: F, t9577: F, t7328: F, t7373: F, t7376: F, t9214: F, t9215: F, t9222: F, t9532: F, t9534: F, t9539: F, t9544: F, t9546: F, t9550: F, t9555: F, t9557: F, t9561: F, t9566: F, t9568: F, t9570: F, t9574: F) -> F {
    let t9578 = t598 * t9577;
    let t9580 = -t9214 - t9215 + F::new(0.32155513588552302729e-2) * t9532 - F::new(0.42874018118069736972e-3) * t9534 + F::new(0.7862023072401038017e-3) * t9539 - F::new(0.31448092289604152068e-3) * t9544 - F::new(0.42874018118069736972e-3) * t9546 - F::new(0.10718504529517434243e-2) * t9550 - F::new(0.18868855373762491241e-2) * t9555 + F::new(0.85748036236139473944e-3) * t9557 - F::new(0.42874018118069736972e-3) * t9561 - F::new(0.21437009059034868486e-3) * t9566 - F::new(0.68598428988911579156e-2) * t9568 + F::new(0.68598428988911579156e-2) * t9570 - t7328 + t9222 + F::new(0.64311027177104605458e-3) * t9574 - F::new(0.47172138434406228102e-2) * t9578 + t7373 - t7376;
    t9580
}
