//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1171/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1171<F: Float>(t3718: F, t6553: F, t12148: F, t2355: F, t1339: F, t1537: F, t46849: F, t590: F, t1441: F, t493: F, t41588: F, t41592: F, t41595: F, t41600: F, t41604: F, t41607: F, t41610: F, t41613: F, t41616: F, t41619: F) -> (F, F, F) {
    let t47790 = t6553 * t3718;
    let t47791 = t2355 * t12148;
    let t47794 = t1537 * t1339 * t46849 * t590;
    let t47800 = t1441 * t493 * t46849 * t590;
    let t47802 = -F::cast_from(0.25561950635947166451e1_f64) * t47794 + F::cast_from(0.9585731488480187419e0_f64) * t41588 - F::cast_from(0.57514388930881124514e0_f64) * t41592 - t41595 + t41600 - t41604 + F::cast_from(0.1022478025437886658e1_f64) * t47800 - t41607 - t41610 + t41613 + t41616 - t41619;
    (t47790, t47791, t47802)
}
