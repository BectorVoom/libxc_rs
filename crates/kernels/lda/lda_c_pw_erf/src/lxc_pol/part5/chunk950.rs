//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 950/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk950<F: Float>(t11328: F, t11333: F, t11338: F, t11340: F, t11341: F, t11342: F, t11343: F, t11344: F, t19987: F, t20035: F, t20037: F, t20039: F, t20041: F, t20043: F, t20044: F, t20048: F, t8202: F) -> (F,) {
    let t20189 = -t19987 + t11328 + t20035 + t20037 + t20039 - t20041 + t11333 - t20043 - t20044 + t11338 + t11340 - t11341 - t11342 - t8202 - t11343 - t11344 - t20048;
    (t20189,)
}
