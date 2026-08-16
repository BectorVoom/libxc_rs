//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1255/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1255(t35874: f64, t35876: f64, t37743: f64, t37744: f64, t40251: f64, t40253: f64, t40255: f64, t40257: f64, t40260: f64, t40262: f64, t40264: f64, t40268: f64, t40270: f64, t40272: f64, t40274: f64, t40277: f64, t40280: f64, t40283: f64) -> f64 {
    let t42023 = 0.31448092289604152069e-2_f64 * t40251 + 0.13719685797782315831e-1_f64 * t40253 - 0.13719685797782315831e-1_f64 * t40255 + 0.68598428988911579156e-2_f64 * t40257 - t37743 - t37744 - 0.1528125e-1_f64 * t40260 + 0.17149607247227894789e-2_f64 * t40262 + 0.17149607247227894789e-2_f64 * t40264 - 0.94344276868812456207e-3_f64 * t40268 - 0.51448821741683684366e-2_f64 * t40270 + 0.68598428988911579156e-2_f64 * t40272 - 0.68598428988911579156e-2_f64 * t40274 + 0.85748036236139473944e-3_f64 * t40277 - 0.51448821741683684368e-2_f64 * t35874 - 0.42874018118069736972e-3_f64 * t40280 + 0.75475421495049964964e-2_f64 * t35876 - 0.21437009059034868486e-2_f64 * t40283;
    t42023
}
