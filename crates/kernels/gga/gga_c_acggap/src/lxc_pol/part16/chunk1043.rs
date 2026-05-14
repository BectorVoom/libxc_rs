//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1043/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1043<F: Float>(t7433: F, t9593: F, t5612: F, t7822: F, t5743: F, t8511: F, t2068: F, t8480: F, t8907: F, t8911: F, t13364: F, t31115: F, t40116: F, t35849: F, t35851: F, t35875: F, t35877: F, t40251: F, t40253: F, t40255: F, t40257: F, t40260: F, t40262: F, t40264: F, t40268: F) -> (F,) {
    let t40270 = t7433 * t9593;
    let t40272 = t7822 * t5612;
    let t40274 = t8511 * t5743;
    let t40277 = t2068 * t8480 * t8907;
    let t40280 = t2068 * t8480 * t8911;
    let t40283 = t31115 * t13364 * t40116;
    let t40285 = 0.15724046144802076034e-2 * t40251 + 0.68598428988911579156e-2 * t40253 - 0.68598428988911579156e-2 * t40255 + 0.34299214494455789578e-2 * t40257 - t35849 - t35851 - 0.7640625e-2 * t40260 + 0.85748036236139473944e-3 * t40262 + 0.85748036236139473944e-3 * t40264 - 0.47172138434406228102e-3 * t40268 - 0.25724410870841842183e-2 * t40270 + 0.34299214494455789578e-2 * t40272 - 0.34299214494455789578e-2 * t40274 + 0.42874018118069736972e-3 * t40277 - t35875 - 0.21437009059034868486e-3 * t40280 + t35877 - 0.10718504529517434243e-2 * t40283;
    (t40285,)
}
