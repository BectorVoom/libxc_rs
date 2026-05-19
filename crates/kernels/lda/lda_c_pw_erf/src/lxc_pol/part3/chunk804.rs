//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 804/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk804<F: Float>(t164: F, t5446: F, t1901: F, t479: F, t41: F, t4713: F, t1905: F, t163: F, t169: F, t2198: F, t299: F, t171: F, t4150: F, t4153: F, t4156: F, t4160: F, t4163: F, t4165: F, t4250: F, t4258: F, t5433: F, t5440: F, t5442: F, t5444: F) -> (F, F) {
    let t5448 = F::cast_from(0.06301081444628223_f64) * t5446 * t164;
    let t5449 = t1901 * t479;
    let t5451 = t41 * t4713;
    let t5455 = F::cast_from(0.06301081444628223_f64) * t1905 * t479;
    let t5459 = F::cast_from(0.017961351015381915_f64) * t169 * t299 * t2198 * t163;
    let t5464 = -F::cast_from(0.005388405304614574_f64) * t169 * t171 * t5433 * t163 - F::cast_from(0.02394846802050922_f64) * t5440 - F::cast_from(0.06301081444628223_f64) * t5442 - F::cast_from(0.031505407223141116_f64) * t5444 + t5448 + F::cast_from(0.06301081444628223_f64) * t5449 - F::cast_from(0.031505407223141116_f64) * t5451 * t164 - t5455 + t5459 + t4258 - F::cast_from(0.04789693604101844_f64) * t4250 - F::cast_from(0.001975389032890948_f64) * t4150 - F::cast_from(0.007901556131563792_f64) * t4153 - F::cast_from(0.0009908551388980995_f64) * t4156 - t4160 - t4163 - t4165;
    (t5451, t5464)
}
