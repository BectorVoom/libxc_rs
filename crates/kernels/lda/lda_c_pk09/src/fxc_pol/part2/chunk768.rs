//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 768/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk768<F: Float>(t2354: F, t4104: F, t1076: F, t2210: F, t2214: F, t3121: F, t3123: F, t4113: F, t4134: F, t4138: F, t4144: F, t4146: F, t4147: F, t4149: F, t7706: F, t7768: F, t7776: F, t7962: F, t98: F) -> (F,) {
    let t9015 = t2354 * t4104;
    let t9018 = t4134 + 0.09983749558483038 * t3121 + 0.09983749558483038 * t3123 + t4138 + t4113 * t2210 / 6.0 + t4144 + t1076 * t7962 / 6.0 - t4146 + t1076 * t7768 / 6.0 + t4147 / 9.0 + t4149 / 9.0 + t4113 * t2214 / 6.0 + t1076 * t7776 / 6.0 + t1076 * t7706 / 6.0 - t9015 * t98 / 6.0;
    (t9018,)
}
