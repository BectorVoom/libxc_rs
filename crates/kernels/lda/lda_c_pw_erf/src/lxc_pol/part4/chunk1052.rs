//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1052/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1052<F: Float>(t1427: F, t5795: F, t5791: F, t656: F, t3912: F, t5798: F, t2260: F, t3915: F, t1217: F, t2281: F, t3704: F, t858: F, t2271: F, t343: F, t2277: F, t5833: F, t668: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t15139 = t5795 * t1427;
    let t15143 = t5791 * t656;
    let t15145 = t5795 * t3912;
    let t15147 = t5798 * t656;
    let t15149 = t2260 * t3915;
    let t15150 = 2e-21 * t15149;
    let t15151 = t2281 * t1217;
    let t15153 = t858 * t3704;
    let t15180 = 32.0 * t2271 * t343;
    let t15193 = 32.0 * t2277 * t343;
    let t15204 = t5833 * t668;
    (t15139, t15143, t15145, t15147, t15150, t15151, t15153, t15180, t15193, t15204)
}
