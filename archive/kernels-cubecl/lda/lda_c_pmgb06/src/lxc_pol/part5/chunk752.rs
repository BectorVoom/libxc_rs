//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 752/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk752<F: Float>(t7065: F, t7085: F, t1291: F, t1296: F, t2238: F, t2241: F, t2255: F, t2722: F, t2730: F, t3625: F, t3632: F, t378: F, t384: F, t5831: F, t5834: F, t7041: F, t7043: F, t7053: F, t7056: F, t7060: F, t74: F, t787: F) -> (F, F) {
    let t7086 = t7065 + t7085;
    let t7088 = -t1291 * t2730 + F::cast_from(4.0_f64) * t1296 * t7056 + F::cast_from(2.0_f64) * t1296 * t7060 - F::cast_from(2.0_f64) * t2238 * t2255 + F::cast_from(4.0_f64) * t5834 * t2241 + F::cast_from(2.0_f64) * t3625 * t2722 - F::cast_from(6.0_f64) * t3632 * t7053 - t378 * t7086 - t7043 * t384 - F::cast_from(2.0_f64) * t5831 * t787 + t7041 * t74;
    (t7086, t7088)
}
