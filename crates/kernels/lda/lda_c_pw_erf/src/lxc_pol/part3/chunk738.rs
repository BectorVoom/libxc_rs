//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 738/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk738<F: Float>(t164: F, t5446: F, t1901: F, t479: F, t41: F, t4713: F, t1905: F, t163: F, t169: F, t2198: F, t299: F, t171: F, t4150: F, t4153: F, t4156: F, t4160: F, t4163: F, t4165: F, t4250: F, t4258: F, t5433: F, t5440: F, t5442: F, t5444: F) -> (F, F) {
    let t5448 = 0.06301081444628223 * t5446 * t164;
    let t5449 = t1901 * t479;
    let t5451 = t41 * t4713;
    let t5455 = 0.06301081444628223 * t1905 * t479;
    let t5459 = 0.017961351015381915 * t169 * t299 * t2198 * t163;
    let t5464 = -0.005388405304614574 * t169 * t171 * t5433 * t163 - 0.02394846802050922 * t5440 - 0.06301081444628223 * t5442 - 0.031505407223141116 * t5444 + t5448 + 0.06301081444628223 * t5449 - 0.031505407223141116 * t5451 * t164 - t5455 + t5459 + t4258 - 0.04789693604101844 * t4250 - 0.001975389032890948 * t4150 - 0.007901556131563792 * t4153 - 0.0009908551388980995 * t4156 - t4160 - t4163 - t4165;
    (t5451, t5464)
}
