//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 933/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk933<F: Float>(t1444: F, t5467: F, t5471: F, t4880: F, t493: F, t5463: F, t10220: F, t176: F, t4885: F, t1820: F, t2938: F, t1919: F, t1083: F, t4865: F, t1981: F, t10230: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12574 = t1444 * t5467 / 9.0;
    let t12576 = 8.0 / 27.0 * t1444 * t5471;
    let t12579 = t493 * t5463 * t4880 / 9.0;
    let t12580 = t10220 * t176;
    let t12583 = 8.0 / 27.0 * t493 * t12580 * t4885;
    let t12584 = t1820 * t2938;
    let t12587 = t493 * t1919 * t12584 / 27.0;
    let t12588 = t4865 * t1083;
    let t12591 = 2.0 / 9.0 * t1981 * t1919 * t12588;
    let t12592 = t10230 * t176;
    (t12574, t12576, t12579, t12583, t12584, t12587, t12588, t12591, t12592)
}
