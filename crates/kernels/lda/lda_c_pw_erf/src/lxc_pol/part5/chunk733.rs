//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 733/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk733<F: Float>(t163: F, t169: F, t2668: F, t299: F, t2379: F, t479: F, t164: F, t7045: F, t171: F, t4175: F, t4254: F, t4260: F, t4261: F, t4265: F, t4272: F, t4275: F, t4276: F, t4279: F, t5468: F, t5472: F, t5477: F, t5479: F, t7032: F, t7287: F) -> (F, F, F, F) {
    let t7294 = t169 * t299 * t2668 * t163;
    let t7298 = t2379 * t479;
    let t7300 = t7045 * t164;
    let t7302 = t4175 - 0.0009908551388980995 * t5468 + 0.013169260219272987 * t5472 - t5477 - 0.007901556131563792 * t5479 - 0.06301081444628223 * t4276 - t4279 + t4254 - t4260 - 0.031505407223141116 * t4261 - t4265 + 0.06301081444628223 * t4272 + t4275 - 0.005388405304614574 * t169 * t171 * t7287 * t163 + 0.008980675507690957 * t7294 - 0.031505407223141116 * t7032 * t164 - 0.031505407223141116 * t7298 + 0.031505407223141116 * t7300;
    (t7294, t7298, t7300, t7302)
}
