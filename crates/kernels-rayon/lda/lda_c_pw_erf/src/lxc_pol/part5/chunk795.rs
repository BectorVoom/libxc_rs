//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 795/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk795(t163: f64, t169: f64, t2668: f64, t299: f64, t2379: f64, t479: f64, t164: f64, t7045: f64, t171: f64, t4175: f64, t4254: f64, t4260: f64, t4261: f64, t4265: f64, t4272: f64, t4275: f64, t4276: f64, t4279: f64, t5468: f64, t5472: f64, t5477: f64, t5479: f64, t7032: f64, t7287: f64) -> (f64, f64, f64, f64) {
    let t7294 = t169 * t299 * t2668 * t163;
    let t7298 = t2379 * t479;
    let t7300 = t7045 * t164;
    let t7302 = t4175 - 0.0009908551388980995_f64 * t5468 + 0.013169260219272987_f64 * t5472 - t5477 - 0.007901556131563792_f64 * t5479 - 0.06301081444628223_f64 * t4276 - t4279 + t4254 - t4260 - 0.031505407223141116_f64 * t4261 - t4265 + 0.06301081444628223_f64 * t4272 + t4275 - 0.005388405304614574_f64 * t169 * t171 * t7287 * t163 + 0.008980675507690957_f64 * t7294 - 0.031505407223141116_f64 * t7032 * t164 - 0.031505407223141116_f64 * t7298 + 0.031505407223141116_f64 * t7300;
    (t7294, t7298, t7300, t7302)
}
