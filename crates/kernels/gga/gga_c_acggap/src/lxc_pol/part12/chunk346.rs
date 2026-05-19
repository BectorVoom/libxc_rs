//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 346/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk346<F: Float>(t1378: F, t85: F, t483: F, t75: F, t288: F, t224: F, t484: F, t229: F, t87: F, t40: F, t276: F, t884: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t1379 = t1378 * t85;
    let t1380 = F::cast_from(0.19751673498613801407e-1_f64) * t1379;
    let t1381 = t483 * t75;
    let t1382 = t1381 * t288;
    let t1383 = F::cast_from(0.5848223622634646207e0_f64) * t1382;
    let t1384 = t224 * t484;
    let t1385 = F::new(4.0) * t1384;
    let t1386 = t229 * t484;
    let t1387 = F::new(4.0) * t1386;
    let t1388 = t1378 * t87;
    let t1389 = t40 * t1388;
    let t1390 = t483 * t276;
    let t1391 = t40 * t1390;
    let t1392 = F::cast_from(0.5848223622634646207e0_f64) * t884;
    (t1380, t1381, t1383, t1385, t1387, t1388, t1389, t1390, t1391, t1392)
}
