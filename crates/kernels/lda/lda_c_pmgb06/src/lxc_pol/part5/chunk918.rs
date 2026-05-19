//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 918/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk918<F: Float>(t11608: F, t1186: F, t1770: F, t5899: F, t1193: F, t4001: F, t4299: F, t794: F, t4320: F, t909: F, t123: F, t317: F, t902: F) -> (F, F, F, F, F) {
    let t11609 = F::cast_from(5.4655730795145296e-05_f64) * t11608;
    let t11611 = t5899 * t1186 * t1770;
    let t11615 = t4001 * t794 * t1193 * t4299;
    let t11617 = t4320 * t909;
    let t11624 = t123 * t4001 * t902 * t317;
    (t11609, t11611, t11615, t11617, t11624)
}
