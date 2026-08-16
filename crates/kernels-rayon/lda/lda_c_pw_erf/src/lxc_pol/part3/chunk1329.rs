//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1329/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1329(t11250: f64, t11254: f64, t11256: f64, t11260: f64, t14429: f64, t14430: f64, t14431: f64, t14432: f64, t14433: f64, t14434: f64, t14436: f64, t14437: f64, t8473: f64, t8477: f64, t8481: f64, t8491: f64, t8505: f64, t8509: f64, t8516: f64) -> f64 {
    let t15295 = t8473 - t8477 + t11250 - t8481 + t14429 - t14430 + t8491 + t14431 - t14432 - t14433 - t14434 - 1.4220018064581168_f64 * t11254 - t8505 + t8509 + t14436 + 9.49086444924727_f64 * t11256 - 1.898172889849454_f64 * t11260 - t14437 + t8516;
    t15295
}
