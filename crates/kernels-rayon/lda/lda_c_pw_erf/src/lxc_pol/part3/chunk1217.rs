//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1217/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1217(t4647: f64, t544: f64, t1524: f64, t2123: f64, t1394: f64, t1982: f64, t1518: f64, t2066: f64, t211: f64, t4703: f64, t595: f64, t14344: f64, t14347: f64, t14350: f64, t14352: f64, t14353: f64, t14354: f64, t14355: f64, t14357: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14359 = 2.0_f64 / 5.0_f64 * t4647 * t544;
    let t14360 = t1524 * t2123;
    let t14361 = 8.0_f64 / 15.0_f64 * t14360;
    let t14363 = 4.0_f64 / 5.0_f64 * t1982 * t1394;
    let t14365 = t211 * t1518 * t2066;
    let t14366 = 4.0_f64 / 45.0_f64 * t14365;
    let t14368 = 2.0_f64 / 5.0_f64 * t4703 * t595;
    let t14369 = -t14344 - t14347 + t14350 - t14352 - t14353 + t14354 + t14355 - t14357 - t14359 - t14361 + t14363 + t14366 - t14368;
    (t14359, t14361, t14363, t14366, t14368, t14369)
}
