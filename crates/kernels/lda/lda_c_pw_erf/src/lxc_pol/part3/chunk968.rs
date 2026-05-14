//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 968/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk968<F: Float>(t12869: F, t11746: F, t519: F, t5256: F, t518: F, t5210: F, t1322: F, t12299: F, t1329: F, t10474: F, t2007: F, t5064: F, t12641: F, t2146: F, t3829: F, t3421: F, t4763: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t12870 = 8.0 / 81.0 * t12869;
    let t12873 = 16.0 / 3.0 * t519 * t5256 * t11746;
    let t12874 = t5210 * t518;
    let t12876 = 16.0 / 15.0 * t12874 * t1322;
    let t12878 = 16.0 / 15.0 * t12299 * t1329;
    let t12880 = 8.0 / 15.0 * t10474 * t2007;
    let t12881 = t5064 * t518;
    let t12883 = 8.0 / 15.0 * t12881 * t2007;
    let t12885 = 16.0 / 15.0 * t12641 * t2007;
    let t12887 = 4.0 / 15.0 * t2146 * t3829;
    let t12889 = 8.0 / 15.0 * t4763 * t3421;
    (t12870, t12873, t12874, t12876, t12878, t12880, t12881, t12883, t12885, t12887, t12889)
}
