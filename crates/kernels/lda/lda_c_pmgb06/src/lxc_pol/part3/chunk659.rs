//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 659/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk659<F: Float>(t1147: F, t123: F, t317: F, t701: F, t1126: F, t740: F, t1312: F, t1316: F, t1317: F, t1323: F, t2180: F, t346: F, t388: F, t3995: F, t3999: F, t4005: F, t4006: F, t4013: F, t4017: F, t4021: F, t4027: F) -> (F, F, F) {
    let t4030 = t123 * t1147 * t701 * t317;
    let t4034 = t123 * t740 * t1126 * t317;
    let t4036 = F::new(0.004067943812504169) * t3995 + t3999 - t4005 + F::new(9.0) * t1316 * t388 * t4006 + F::new(9.0) * t1316 * t1312 * t1317 - F::new(2.0) * t346 * t1323 * t4013 + F::new(9.0) * t1316 * t388 * t4017 + F::new(18.0) * t2180 * t388 * t4021 - t4027 + F::new(0.5945049527603057) * t4030 - F::new(0.16213771438917426) * t4034;
    (t4030, t4034, t4036)
}
