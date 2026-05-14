//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1022/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1022<F: Float>(t2007: F, t3873: F, t554: F, t3869: F, t125: F, t3966: F, t544: F, t1173: F, t8503: F, t3: F, t3014: F, t8498: F, t1993: F, t3865: F, t6461: F, t1978: F, t3854: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10240 = t554 * t2007 * t3873;
    let t10243 = t554 * t2007 * t3869;
    let t10245 = t3966 * t125;
    let t10246 = t10245 * t544;
    let t10250 = t8503 * t1173;
    let t10254 = t3014 * t3;
    let t10258 = t8498 * t1173;
    let t10263 = t1993 * t6461 * t3865;
    let t10265 = t1978 * t3854;
    (t10240, t10243, t10245, t10246, t10250, t10254, t10258, t10263, t10265)
}
