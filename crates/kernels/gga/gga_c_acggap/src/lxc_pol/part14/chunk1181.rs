//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1181/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1181<F: Float>(t2068: F, t8480: F, t8907: F, t8911: F, t13364: F, t31115: F, t40116: F, t35849: F, t35851: F, t35875: F, t35877: F, t40251: F, t40253: F, t40255: F, t40257: F, t40260: F, t40262: F, t40264: F, t40268: F, t40270: F, t40272: F, t40274: F) -> F {
    let t40277 = t2068 * t8480 * t8907;
    let t40280 = t2068 * t8480 * t8911;
    let t40283 = t31115 * t13364 * t40116;
    let t40285 = F::new(0.15724046144802076034e-2) * t40251 + F::new(0.68598428988911579156e-2) * t40253 - F::new(0.68598428988911579156e-2) * t40255 + F::new(0.34299214494455789578e-2) * t40257 - t35849 - t35851 - F::new(0.7640625e-2) * t40260 + F::new(0.85748036236139473944e-3) * t40262 + F::new(0.85748036236139473944e-3) * t40264 - F::new(0.47172138434406228102e-3) * t40268 - F::new(0.25724410870841842183e-2) * t40270 + F::new(0.34299214494455789578e-2) * t40272 - F::new(0.34299214494455789578e-2) * t40274 + F::new(0.42874018118069736972e-3) * t40277 - t35875 - F::new(0.21437009059034868486e-3) * t40280 + t35877 - F::new(0.10718504529517434243e-2) * t40283;
    t40285
}
