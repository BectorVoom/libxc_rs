//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 872/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk872<F: Float>(t60: F, t20: F, t2394: F, t63: F, t697: F, t2404: F, t700: F, t209: F, t2403: F, t2410: F, t8747: F, t698: F, t2399: F, t2406: F, t2412: F, t4879: F, t696: F, t702: F, t75: F) -> (F,) {
    let t70 = 0.0 < t60;
    let t9194 = t2394 * t20;
    let t9195 = t63 * t9194;
    let t9202 = t697 * t697;
    let t9203 = 1.0 / t9202;
    let t9204 = t2404 * t700;
    let t9206 = t209 * t9203 * t9204;
    let t9209 = t2403 * t700;
    let t9211 = t209 * t9209 * t2410;
    let t9215 = piecewise3(t70, t8747, -t8747);
    let t9217 = t209 * t698 * t9215;
    let t9220 = -455.0 / 1296.0 * t63 * t4879 * t75 - 35.0 / 144.0 * t9195 * t702 - 7.0 / 48.0 * t2399 * t2406 + 7.0 / 96.0 * t2399 * t2412 - t696 * t9206 / 16.0 + t696 * t9211 / 16.0 - t696 * t9217 / 96.0;
    (t9220,)
}
