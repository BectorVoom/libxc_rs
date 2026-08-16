//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 782/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk782(t56: f64, t7306: f64, t38: f64, t370: f64, t2448: f64, t780: f64, t64: f64, t35: f64, t1282: f64, t7277: f64, t3505: f64, t3513: f64, t3515: f64, t3517: f64, t3521: f64, t3523: f64, t3525: f64, t360: f64, t63: f64, t7278: f64, t7283: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7307 = t56 * t7306;
    let t7309 = 2.923025_f64 * t38 * t7307;
    let t7310 = t370 * t7306;
    let t7313 = t780 * t2448;
    let t7317 = t64 * t7306;
    let t7318 = t35 * t7317;
    let t7321 = t1282 * t7277;
    let t7322 = t35 * t7321;
    let t7325 = -t3505 + t3513 - 29.3808_f64 * t63 * t7278 - t7283 - t7309 - 1.46904_f64 * t63 * t7310 + 9.0_f64 / 2.0_f64 * t360 * t35 * t7313 - t3515 - t3517 - t3521 - t3523 + t3525 - t360 * t7318 / 2.0_f64 - 6.0_f64 * t360 * t7322;
    (t7307, t7309, t7310, t7313, t7317, t7318, t7321, t7322, t7325)
}
