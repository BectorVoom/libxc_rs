//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1266/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1266<F: Float>(t27819: F, t5120: F, t6119: F, t729: F, t747: F, t24438: F, t30990: F, t684: F, t24437: F, t27468: F, t27775: F, t31019: F, t122003: F, t24432: F, t6118: F, t124267: F, t124270: F, t124273: F, t124277: F, t124279: F, t124282: F, t124287: F) -> (F, F, F, F, F, F) {
    let t124292 = t27819 * t729 * t6119 * t5120 * t747;
    let t124296 = t27819 * t24438 * t30990 * t684;
    let t124300 = t24437 * t24438 * t27468 * t27775;
    let t124304 = t24437 * t24438 * t31019 * t684;
    let t124307 = t6118 * t24432 * t122003;
    let t124309 = -2.0 / 9.0 * t124267 - 2.0 / 9.0 * t124270 + 4.0 / 9.0 * t124273 + t124277 / 18.0 + 2.0 / 27.0 * t124279 + t124282 / 9.0 + 5.0 / 16.0 * t124287 - t124292 / 8.0 + t124296 / 24.0 - t124300 / 18.0 - t124304 / 36.0 - 2.0 / 9.0 * t124307;
    (t124292, t124296, t124300, t124304, t124307, t124309)
}
