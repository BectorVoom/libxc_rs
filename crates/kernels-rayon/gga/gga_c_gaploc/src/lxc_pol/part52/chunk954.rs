//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 954/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk954(t39657: f64, t42114: f64, t44590: f64, t44592: f64, t44595: f64, t44596: f64, t44597: f64, t44598: f64, t44599: f64, t44600: f64, t493: f64, t105: f64, t14268: f64, t380: f64, t419: f64, t44301: f64, t44302: f64, t44305: f64, t44306: f64, t44309: f64, t44313: f64, t44316: f64, t44319: f64, t44322: f64, t44325: f64, t46878: f64, t46884: f64, t46887: f64, t46889: f64, t46892: f64, t492: f64) -> (f64, f64, f64) {
    let t49841 = t44590 - t44592 + 2.0_f64 * t42114 - 2.0_f64 * t39657 + t44595 + t44596 - t44597 + t44598 - t44599 - t44600;
    let t49842 = t493 * t49841;
    let t49851 = t44301 + t44302 - t44305 - t44306 - t44309 + t44313 + t44316 - 0.37940008847568199465e-1_f64 * t380 * t14268 - 0.28455006635676149599e-1_f64 * t419 * t14268 - 0.28455006635676149599e-1_f64 * t105 * t492 * t49842 - t44319 + t44322 - t44325 - 0.47425011059460249332e-2_f64 * t46878 - 0.47425011059460249332e-2_f64 * t46884 - 0.47425011059460249332e-2_f64 * t46887 + 0.47425011059460249332e-2_f64 * t46889 + 0.47425011059460249332e-2_f64 * t46892;
    (t49841, t49842, t49851)
}
