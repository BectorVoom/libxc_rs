//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 954/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk954<F: Float>(t39657: F, t42114: F, t44590: F, t44592: F, t44595: F, t44596: F, t44597: F, t44598: F, t44599: F, t44600: F, t493: F, t105: F, t14268: F, t380: F, t419: F, t44301: F, t44302: F, t44305: F, t44306: F, t44309: F, t44313: F, t44316: F, t44319: F, t44322: F, t44325: F, t46878: F, t46884: F, t46887: F, t46889: F, t46892: F, t492: F) -> (F, F, F) {
    let t49841 = t44590 - t44592 + F::new(2.0) * t42114 - F::new(2.0) * t39657 + t44595 + t44596 - t44597 + t44598 - t44599 - t44600;
    let t49842 = t493 * t49841;
    let t49851 = t44301 + t44302 - t44305 - t44306 - t44309 + t44313 + t44316 - F::new(0.37940008847568199465e-1) * t380 * t14268 - F::new(0.28455006635676149599e-1) * t419 * t14268 - F::new(0.28455006635676149599e-1) * t105 * t492 * t49842 - t44319 + t44322 - t44325 - F::new(0.47425011059460249332e-2) * t46878 - F::new(0.47425011059460249332e-2) * t46884 - F::new(0.47425011059460249332e-2) * t46887 + F::new(0.47425011059460249332e-2) * t46889 + F::new(0.47425011059460249332e-2) * t46892;
    (t49841, t49842, t49851)
}
