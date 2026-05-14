//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1012/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1012<F: Float>(t7111: F, t8648: F, t8650: F, t7127: F, t7157: F, t7159: F, t86: F, t9904: F, t4873: F, t5039: F, t9914: F, t9915: F, t9916: F, t9917: F, t108: F, t9886: F, t9909: F, t9913: F) -> (F, F, F, F, F, F, F, F) {
    let t9918 = 36.0 * t7111;
    let t9919 = 12.0 * t8648;
    let t9920 = 12.0 * t8650;
    let t9921 = 0.35089341735807877242e1 * t7127;
    let t9922 = 0.17544670867903938621e1 * t7157;
    let t9923 = 0.51947577317044391276e2 * t7159;
    let t9924 = t9904 * t86;
    let t9925 = 0.19751673498613801407e-1 * t9924;
    let t9926 = -t9914 - t9915 + t9916 + t9917 + t9918 - t9919 - t9920 + t9921 + t4873 - t9922 - t9923 + t5039 + t9925;
    let t9929 = (t9886 + t9909 + t9913 + t9926) * t108;
    (t9918, t9919, t9920, t9921, t9922, t9923, t9925, t9929)
}
