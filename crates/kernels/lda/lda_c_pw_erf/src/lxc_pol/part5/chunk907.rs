//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 907/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk907<F: Float>(t164: F, t8756: F, t4137: F, t479: F, t8832: F, t1159: F, t1590: F, t695: F, t1198: F, t4263: F, t458: F, t1191: F, t163: F, t169: F, t616: F) -> (F, F, F, F, F, F, F, F) {
    let t9178 = F::new(0.0014238371845981686) * t8756 * t164;
    let t9180 = F::new(0.0004746123948660562) * t4137 * t479;
    let t9181 = t8832 * t164;
    let t9192 = t1159 * t479;
    let t9195 = F::new(0.3780648866776934) * t695 * t1590;
    let t9203 = t1198 * t1590;
    let t9206 = F::new(0.12602162889256446) * t458 * t4263;
    let t9211 = t169 * t1191 * t616 * t163;
    (t9178, t9180, t9181, t9192, t9195, t9203, t9206, t9211)
}
