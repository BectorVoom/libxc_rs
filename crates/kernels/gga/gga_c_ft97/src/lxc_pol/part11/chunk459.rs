//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 459/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk459<F: Float>(t2331: F, t2465: F, t247: F, t2470: F, t2527: F, t2570: F, t2617: F, t2619: F, t263: F, t719: F, t771: F, t342: F, t630: F, t784: F, t294: F, t668: F) -> (F, F, F) {
    let t2624 = -t2331 * t263 - t2465 * t263 - t247 * t2617 - 2.0 * t719 * t771 - 4.0 * t2470 - 2.0 * t2527 + 4.0 * t2570 + 2.0 * t2619;
    let t2638 = t342 * t630 * t784 / 12.0;
    let t2639 = t294 * t668;
    (t2624, t2638, t2639)
}
