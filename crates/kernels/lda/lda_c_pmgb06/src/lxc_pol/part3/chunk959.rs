//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 959/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk959<F: Float>(t1438: F, t154: F, t12398: F, t5077: F, t3098: F, t5083: F, t12973: F, t12974: F, t12975: F, t12976: F, t12977: F, t12978: F, t12979: F, t12983: F, t12986: F, t12990: F) -> (F, F, F) {
    let t12991 = t154 * t1438;
    let t12994 = 2.0 / 5.0 * t5077 * t12991 * t12398;
    let t12995 = t154 * t3098;
    let t12998 = 2.0 / 3.0 * t5083 * t12995 * t12398;
    let t12999 = t12973 + t12974 + t12975 + t12976 + t12977 + t12978 + t12979 - t12983 + t12986 + t12990 + t12994 - t12998;
    (t12994, t12998, t12999)
}
