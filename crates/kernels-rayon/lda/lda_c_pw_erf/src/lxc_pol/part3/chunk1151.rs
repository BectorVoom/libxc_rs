//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1151/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1151(t2146: f64, t3763: f64, t3900: f64, t4763: f64, t3799: f64, t4738: f64, t13453: f64, t13458: f64, t13463: f64, t13465: f64, t13466: f64, t13467: f64, t13469: f64, t13471: f64, t13475: f64, t13477: f64) -> (f64, f64, f64, f64) {
    let t13478 = t2146 * t3763;
    let t13479 = 8.0_f64 / 135.0_f64 * t13478;
    let t13480 = t4763 * t3900;
    let t13481 = 16.0_f64 / 15.0_f64 * t13480;
    let t13483 = 4.0_f64 / 5.0_f64 * t4738 * t3799;
    let t13484 = -t13453 + t13458 + t13463 - t13465 + t13466 - t13467 + t13469 + t13471 - t13475 - t13477 - t13479 - t13481 - t13483;
    (t13479, t13481, t13483, t13484)
}
