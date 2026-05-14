//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1255/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1255<F: Float>(t11535: F, t1291: F, t1296: F, t1297: F, t1309: F, t18796: F, t18804: F, t18807: F, t18823: F, t18835: F, t18842: F, t18869: F, t2238: F, t2241: F, t2255: F, t2722: F, t2730: F, t3622: F, t3632: F, t378: F, t384: F, t5831: F, t5880: F, t7043: F, t7086: F, t787: F, t8399: F, t8413: F) -> (F,) {
    let t18876 = 24.0 * t8413 * t2722 * t1297 - 24.0 * t3632 * t2241 * t2255 - 2.0 * t2238 * t5880 + 2.0 * t8399 * t2722 - 2.0 * t18796 * t384 - t7043 * t1309 - 2.0 * t11535 * t787 - 4.0 * t5831 * t2255 + 4.0 * t1296 * t18804 + 2.0 * t18807 * t1297 - t378 * (t18823 + t18835 + t18842 + t18869) - t3622 * t2730 - 2.0 * t1291 * t7086;
    (t18876,)
}
