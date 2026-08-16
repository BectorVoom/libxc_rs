//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1075/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1075(t8464: f64, t8482: f64, t8486: f64, t14435: f64, t11250: f64, t11254: f64, t11256: f64, t14432: f64, t14433: f64, t14437: f64, t14439: f64, t8469: f64, t8473: f64, t8477: f64, t8481: f64, t8491: f64, t8505: f64, t8509: f64, t8516: f64) -> (f64, f64, f64, f64, f64) {
    let t20090 = 0.0005696928233656539_f64 * t8464;
    let t20091 = 3.5089340384731225_f64 * t8482;
    let t20092 = 51.94726769812759_f64 * t8486;
    let t20094 = 180.0_f64 * t14435;
    let t20096 = -t20090 + t8469 + t8473 - t8477 + t11250 - t8481 + t20091 - t20092 + t8491 + t14432 + t14433 - 0.4740006021527056_f64 * t11254 - t8505 + t8509 + t20094 + 3.1636214830824234_f64 * t11256 + t14437 + t8516 + t14439;
    (t20090, t20091, t20092, t20094, t20096)
}
