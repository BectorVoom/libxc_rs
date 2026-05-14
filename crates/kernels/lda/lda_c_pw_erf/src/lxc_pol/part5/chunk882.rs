//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 882/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk882<F: Float>(t3990: F, t851: F, t256: F, t3932: F, t850: F, t2260: F, t3927: F, t1432: F, t2252: F, t1427: F, t5795: F, t5791: F, t656: F, t3912: F, t5798: F, t3915: F) -> (F, F, F, F, F, F, F, F, F) {
    let t15111 = t851 * t3990;
    let t15123 = t850 * t3932 * t256;
    let t15125 = t2260 * t3927;
    let t15138 = t2252 * t1432 * t256;
    let t15139 = t5795 * t1427;
    let t15140 = 0.36466666666666664 * t15139;
    let t15143 = t5791 * t656;
    let t15144 = 4.0 / 3.0 * t15143;
    let t15145 = t5795 * t3912;
    let t15146 = 2e-21 * t15145;
    let t15147 = t5798 * t656;
    let t15149 = t2260 * t3915;
    (t15111, t15123, t15125, t15138, t15140, t15144, t15146, t15147, t15149)
}
