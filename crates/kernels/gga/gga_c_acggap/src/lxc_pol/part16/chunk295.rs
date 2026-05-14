//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 295/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk295<F: Float>(t1378: F, t85: F, t483: F, t75: F, t288: F, t224: F, t484: F, t229: F, t87: F, t40: F, t276: F, t884: F, t906: F, t764: F, t774: F, t782: F, t905: F, t914: F) -> (F, F, F, F, F, F, F, F) {
    let t1379 = t1378 * t85;
    let t1380 = 0.19751673498613801407e-1 * t1379;
    let t1381 = t483 * t75;
    let t1382 = t1381 * t288;
    let t1383 = 0.5848223622634646207e0 * t1382;
    let t1384 = t224 * t484;
    let t1385 = 4.0 * t1384;
    let t1386 = t229 * t484;
    let t1387 = 4.0 * t1386;
    let t1388 = t1378 * t87;
    let t1389 = t40 * t1388;
    let t1390 = t483 * t276;
    let t1391 = t40 * t1390;
    let t1392 = 0.5848223622634646207e0 * t884;
    let t1393 = 4.0 * t906;
    let t1394 = t1380 - t1383 + t1385 - t1387 + t1389 + t1391 + t914 - t1392 - t905 - t1393 - t764 + t774 + t782;
    (t1381, t1382, t1384, t1386, t1388, t1390, t1391, t1394)
}
