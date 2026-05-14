//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 388/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk388<F: Float>(t209: F, t2410: F, t698: F, t2394: F, t2399: F, t2406: F, t63: F, t696: F, t702: F, t75: F, t706: F, t124: F, t691: F, t138: F, t86: F, t66: F, t747: F) -> (F, F, F, F, F, F) {
    let t2412 = t209 * t698 * t2410;
    let t2415 = 35.0 / 432.0 * t63 * t2394 * t75 + 7.0 / 144.0 * t2399 * t702 + t696 * t2406 / 48.0 - t696 * t2412 / 96.0;
    let t2416 = t2415 * t706;
    let t2421 = t691 * t124;
    let t2423 = t86 * t2421 * t138;
    let t2425 = t66 * t747;
    (t2412, t2415, t2416, t2421, t2423, t2425)
}
