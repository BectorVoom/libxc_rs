//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1198/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1198(t10997: f64, t10999: f64, t11000: f64, t11002: f64, t11003: f64, t11066: f64, t11073: f64, t14939: f64, t14942: f64, t14944: f64, t14947: f64, t8482: f64, t8519: f64, t8520: f64, t8526: f64, t8543: f64) -> f64 {
    let t21720 = t8482 - t10997 - t8519 + 0.03253074390090522_f64 * t14939 - 120.0_f64 * t8520 - 1.7544670867903938_f64 * t14942 - 51.94757731704439_f64 * t14944 - 1.7544670867903938_f64 * t14947 + t8526 + t10999 + t11000 + t11002 - t11003 + 60.0_f64 * t8543 - t11066 - t11073;
    t21720
}
