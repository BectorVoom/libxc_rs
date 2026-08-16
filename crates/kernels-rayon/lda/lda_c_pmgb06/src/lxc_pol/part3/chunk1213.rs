//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1213/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1213(t13315: f64, t13318: f64, t13322: f64, t13323: f64, t13324: f64, t13327: f64, t13328: f64, t13421: f64, t13423: f64, t13425: f64, t13427: f64, t10720: f64, t10727: f64, t10732: f64, t10735: f64, t13429: f64, t13431: f64, t13433: f64, t13435: f64, t13438: f64, t13440: f64, t13453: f64, t13455: f64) -> (f64, f64) {
    let t14424 = t13315 - t13318 + t13322 - t13323 - t13324 + t13327 - t13328 + t13421 + t13423 + t13425 + t13427;
    let t14428 = 4.0_f64 * t10720 + t10727 + 4.0_f64 / 3.0_f64 * t10732 + 4.0_f64 * t10735 - t13429 - t13431 + t13433 - t13435 - t13438 + t13440 + t13453 - t13455;
    (t14424, t14428)
}
