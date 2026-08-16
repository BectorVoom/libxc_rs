//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1760/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1760(t90400: f64, t90456: f64, t90478: f64, t90497: f64, t1179: f64, t1188: f64, t1196: f64, t6474: f64, t68952: f64, t90349: f64, t90351: f64, t90356: f64, t90361: f64, t90364: f64, t90367: f64, t90370: f64, t90373: f64, t90375: f64, t90377: f64) -> (f64, f64, f64, f64) {
    let t90499 = t90400 + t90456 + t90478 + t90497;
    let t90503 = 0.5848223622634646207e0_f64 * t1196 * t1179 * t90499 * t1188;
    let t90505 = 0.96491876992155210402e2_f64 * t68952 * t6474;
    let t90506 = -t90349 + t90351 - t90356 - t90361 - t90364 - t90367 + t90370 + t90373 - t90375 - t90377 - t90503 + t90505;
    (t90499, t90503, t90505, t90506)
}
