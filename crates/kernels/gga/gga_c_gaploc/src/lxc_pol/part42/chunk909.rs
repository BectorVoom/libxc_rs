//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 909/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk909<F: Float>(t11359: F, t2492: F, t4752: F, t3377: F, t38181: F, t41884: F, t11549: F, t20535: F, t2478: F, t38019: F, t544: F, t9287: F) -> (F, F, F, F, F) {
    let t46311 = F::cast_from(0.28600391961480341335e1_f64) * t11359 * t4752 * t2492;
    let t46316 = F::cast_from(0.10725146985555128001e1_f64) * t38181 * t3377;
    let t46327 = F::cast_from(0.71500979903700853339e0_f64) * t41884;
    let t46331 = t20535 * t11549 * t2478;
    let t46335 = t544 * t38019 * t9287;
    (t46311, t46316, t46327, t46331, t46335)
}
