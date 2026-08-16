//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1181/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1181(t2068: f64, t8480: f64, t8907: f64, t8911: f64, t13364: f64, t31115: f64, t40116: f64, t35849: f64, t35851: f64, t35875: f64, t35877: f64, t40251: f64, t40253: f64, t40255: f64, t40257: f64, t40260: f64, t40262: f64, t40264: f64, t40268: f64, t40270: f64, t40272: f64, t40274: f64) -> f64 {
    let t40277 = t2068 * t8480 * t8907;
    let t40280 = t2068 * t8480 * t8911;
    let t40283 = t31115 * t13364 * t40116;
    let t40285 = 0.15724046144802076034e-2_f64 * t40251 + 0.68598428988911579156e-2_f64 * t40253 - 0.68598428988911579156e-2_f64 * t40255 + 0.34299214494455789578e-2_f64 * t40257 - t35849 - t35851 - 0.7640625e-2_f64 * t40260 + 0.85748036236139473944e-3_f64 * t40262 + 0.85748036236139473944e-3_f64 * t40264 - 0.47172138434406228102e-3_f64 * t40268 - 0.25724410870841842183e-2_f64 * t40270 + 0.34299214494455789578e-2_f64 * t40272 - 0.34299214494455789578e-2_f64 * t40274 + 0.42874018118069736972e-3_f64 * t40277 - t35875 - 0.21437009059034868486e-3_f64 * t40280 + t35877 - 0.10718504529517434243e-2_f64 * t40283;
    t40285
}
