//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 849/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk849<F: Float>(t5862: F, t5879: F, t1291: F, t1296: F, t1297: F, t1309: F, t2238: F, t2241: F, t2255: F, t3622: F, t3625: F, t3632: F, t378: F, t384: F, t5829: F, t5831: F, t5834: F, t5843: F, t5846: F, t5849: F, t74: F, t787: F) -> (F, F) {
    let t5880 = t5862 + t5879;
    let t5882 = -F::new(2.0) * t1291 * t2255 + F::new(4.0) * t1296 * t5846 + F::new(2.0) * t1296 * t5849 + F::new(2.0) * t5834 * t1297 - t2238 * t1309 + F::new(4.0) * t3625 * t2241 - t3622 * t787 - F::new(6.0) * t3632 * t5843 - t378 * t5880 - F::new(2.0) * t5831 * t384 + t5829 * t74;
    (t5880, t5882)
}
