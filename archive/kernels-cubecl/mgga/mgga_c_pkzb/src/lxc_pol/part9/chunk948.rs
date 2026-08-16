//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 948/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk948<F: Float>(t7335: F, t5520: F, t5522: F, t5525: F, t7352: F, t7357: F, t672: F, t665: F, t1861: F, t2759: F, t667: F, t1867: F, t2754: F) -> (F, F, F, F, F, F) {
    let t7359 = F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t7335;
    let t7360 = -t5520 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t5522 - t5525 / F::cast_from(3.0_f64) + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t7357 - t7359 + t7352;
    let t7361 = t672 * t7360;
    let t7363 = t665 * t7360;
    let t7365 = t1861 * t2759;
    let t7366 = t7365 * t667;
    let t7368 = t2754 * t1867;
    (t7360, t7361, t7363, t7365, t7366, t7368)
}
