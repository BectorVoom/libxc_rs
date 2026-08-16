//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 804/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk804(t164: f64, t5446: f64, t1901: f64, t479: f64, t41: f64, t4713: f64, t1905: f64, t163: f64, t169: f64, t2198: f64, t299: f64, t171: f64, t4150: f64, t4153: f64, t4156: f64, t4160: f64, t4163: f64, t4165: f64, t4250: f64, t4258: f64, t5433: f64, t5440: f64, t5442: f64, t5444: f64) -> (f64, f64) {
    let t5448 = 0.06301081444628223_f64 * t5446 * t164;
    let t5449 = t1901 * t479;
    let t5451 = t41 * t4713;
    let t5455 = 0.06301081444628223_f64 * t1905 * t479;
    let t5459 = 0.017961351015381915_f64 * t169 * t299 * t2198 * t163;
    let t5464 = -0.005388405304614574_f64 * t169 * t171 * t5433 * t163 - 0.02394846802050922_f64 * t5440 - 0.06301081444628223_f64 * t5442 - 0.031505407223141116_f64 * t5444 + t5448 + 0.06301081444628223_f64 * t5449 - 0.031505407223141116_f64 * t5451 * t164 - t5455 + t5459 + t4258 - 0.04789693604101844_f64 * t4250 - 0.001975389032890948_f64 * t4150 - 0.007901556131563792_f64 * t4153 - 0.0009908551388980995_f64 * t4156 - t4160 - t4163 - t4165;
    (t5451, t5464)
}
