//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1228/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1228<F: Float>(t2249: F, t384: F, t387: F, t10551: F, t10582: F, t10648: F, t11717: F, t11740: F, t1312: F, t1316: F, t14247: F, t14257: F, t14270: F, t14497: F, t14511: F, t14521: F, t14551: F, t2180: F, t2311: F, t295: F, t346: F, t3659: F, t4053: F, t4358: F, t4398: F, t5583: F, t5721: F, t61: F, t790: F, t8189: F, t8202: F, t8206: F, t8208: F, t8211: F) -> F {
    let t14561 = t387 * t384 * t2249;
    let t14564 = F::cast_from(0.17961351015381913_f64) * t8189 - F::cast_from(0.01197423401025461_f64) * t8202 - F::cast_from(0.03592270203076383_f64) * t8206 + F::cast_from(0.585406996056892_f64) * t8208 + t8211 + (t11717 + t11740 + t14247 + t14257) * t295 + F::new(3.0) * t1316 * t3659 * t2311 - F::new(3.0) * t346 * t4398 * t4053 + F::new(9.0) * t1316 * t1312 * t5721 - F::new(6.0) * t346 * t14270 * t10551 + (t14497 + t14511 + t14521 + t14551) * t61 + F::new(18.0) * t2180 * t790 * t10648 - F::new(9.0) * t5583 * t10582 + F::new(18.0) * t4358 * t14561;
    t14564
}
