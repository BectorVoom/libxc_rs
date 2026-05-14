//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1272/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1272<F: Float>(t23893: F, t86: F, t19413: F, t19415: F, t2267: F, t2858: F, t7591: F, t2788: F, t4970: F, t4994: F, t963: F, t1422: F, t2484: F, t2271: F, t7136: F, t1524: F, t2747: F) -> (F, F, F, F, F, F, F, F, F) {
    let t23895 = 0.19751673498613801407e-1 * t23893 * t86;
    let t23896 = 60.0 * t19413;
    let t23897 = 36.0 * t19415;
    let t23900 = 36.0 * t2858 * t2267 * t7591;
    let t23901 = t2788 * t4970;
    let t23902 = 0.16265371950452609763e-1 * t23901;
    let t23903 = t963 * t4994;
    let t23904 = 0.35089341735807877242e1 * t23903;
    let t23906 = 96.0 * t1422 * t2484;
    let t23907 = t2271 * t7136;
    let t23909 = t2747 * t1524;
    (t23895, t23896, t23897, t23900, t23902, t23904, t23906, t23907, t23909)
}
