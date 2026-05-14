//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 213/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk213<F: Float>(t60: F, t20: F, t66: F, t63: F, t72: F, t684: F, t209: F, t691: F, t75: F, t78: F, t124: F, t138: F, t86: F, t95: F, t96: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t70 = 0.0 < t60;
    let t695 = t66 * t20;
    let t696 = t63 * t695;
    let t697 = t72 * t72;
    let t698 = 1.0 / t697;
    let t700 = piecewise3(t70, t684, -t684);
    let t702 = t209 * t698 * t700;
    let t705 = -7.0 / 288.0 * t63 * t691 * t75 - t696 * t702 / 96.0;
    let t706 = 1.0 / t78;
    let t707 = t705 * t706;
    let t710 = t66 * t124;
    let t712 = t86 * t710 * t138;
    let t717 = 1.0 / t96 / t95;
    (t696, t697, t698, t700, t702, t705, t706, t707, t712, t717)
}
