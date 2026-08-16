//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 919/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk919<F: Float>(t13402: F, t2487: F, t6985: F, t34400: F, t34401: F, t46362: F, t10525: F, t10526: F, t46115: F, t46103: F, t6716: F, t6717: F) -> (F, F, F, F) {
    let t46520 = t2487 * t6985 * t13402;
    let t46521 = F::cast_from(0.25561950635947166451e0_f64) * t46520;
    let t46526 = F::cast_from(0.13803453343411469884e3_f64) * t34400 * t34401 * t46362;
    let t46529 = F::cast_from(0.42900587942220512002e1_f64) * t10525 * t10526 * t46115;
    let t46535 = F::cast_from(0.69017266717057349418e1_f64) * t6716 * t6717 * t46103;
    (t46521, t46526, t46529, t46535)
}
