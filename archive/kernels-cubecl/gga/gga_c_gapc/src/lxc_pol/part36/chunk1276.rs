//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 1276/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk1276<F: Float>(t12331: F, t12434: F, t10526: F, t3537: F, t12327: F, t575: F, t687: F, t12339: F, t23726: F, t12346: F, t4908: F, t1616: F) -> (F, F, F, F, F, F, F) {
    let t37336 = F::cast_from(4.0_f64) * t12331;
    let t37337 = F::cast_from(2.0_f64) * t12434;
    let t37339 = F::cast_from(4.0_f64) * t10526 * t3537;
    let t37340 = t12327 * t575;
    let t37342 = F::cast_from(2.0_f64) * t37340 * t687;
    let t37344 = F::cast_from(12.0_f64) * t23726 * t12339;
    let t37346 = F::cast_from(4.0_f64) * t4908 * t12346;
    let t37347 = t3537 * t3537;
    let t37349 = F::cast_from(4.0_f64) * t1616 * t37347;
    (t37336, t37337, t37339, t37342, t37344, t37346, t37349)
}
