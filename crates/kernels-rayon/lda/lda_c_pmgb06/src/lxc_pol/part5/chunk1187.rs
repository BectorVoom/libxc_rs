//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1187/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1187(t38: f64, t5980: f64, t776: f64, t342: f64, t7317: f64, t11230: f64, t21410: f64, t21423: f64, t21439: f64, t21442: f64, t21445: f64, t2209: f64, t2229: f64, t2448: f64, t5740: f64, t63: f64, t6989: f64, t7277: f64, t8245: f64) -> (f64, f64, f64) {
    let t21448 = 17.53815_f64 * t38 * t776 * t5980;
    let t21451 = 5.84605_f64 * t38 * t7317 * t342;
    let t21452 = -88.1424_f64 * t11230 * t21410 - t21423 + 176.2848_f64 * t63 * t8245 * t7277 * t342 - 88.1424_f64 * t63 * t6989 * t2209 + 17.62848_f64 * t63 * t5740 * t2448 + 17.62848_f64 * t63 * t2229 * t5980 + t21439 - t21442 + t21445 + t21448 + t21451;
    (t21448, t21451, t21452)
}
