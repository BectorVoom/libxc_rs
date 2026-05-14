//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 960/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk960<F: Float>(t2029: F, t4119: F, t224: F, t4753: F, t1447: F, t5176: F, t1989: F, t3226: F, t2007: F, t3213: F, t131: F, t1767: F, t129: F, t2012: F, t10318: F, t806: F) -> (F, F, F, F, F, F, F, F) {
    let t11813 = t2029 * t4119;
    let t11821 = t4753 * t224;
    let t11830 = t1447 * t5176;
    let t11832 = t3226 * t1989;
    let t11860 = t3213 * t2007;
    let t11862 = t131 * t1767;
    let t11864 = t129 * t11862 * t2012;
    let t11866 = t10318 * t806;
    (t11813, t11821, t11830, t11832, t11860, t11862, t11864, t11866)
}
