//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 671/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk671<F: Float>(t1191: F, t163: F, t169: F, t234: F, t164: F, t4137: F, t1590: F, t466: F, t2908: F, t148: F, t1203: F, t479: F, t4100: F, t1198: F, t458: F, t1159: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t4258 = 0.0878110494085338 * t169 * t1191 * t234 * t163;
    let t4259 = t4137 * t164;
    let t4260 = 0.00011865309871651405 * t4259;
    let t4261 = t466 * t1590;
    let t4263 = t2908 * t163;
    let t4265 = 0.031505407223141116 * t148 * t4263;
    let t4268 = t1203 * t479;
    let t4270 = t4100 * t164;
    let t4272 = t1198 * t479;
    let t4275 = 0.09451622166942335 * t458 * t1590;
    let t4276 = t1159 * t164;
    (t4258, t4259, t4260, t4261, t4263, t4265, t4268, t4270, t4272, t4275, t4276)
}
