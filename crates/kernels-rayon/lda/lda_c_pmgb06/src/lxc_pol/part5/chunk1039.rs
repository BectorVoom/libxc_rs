//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1039/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1039(t1423: f64, t7551: f64, t132: f64, t435: f64, t7735: f64, t1447: f64, t7559: f64, t7563: f64, t7517: f64, t15739: f64, t11864: f64, t13788: f64, t2648: f64, t439: f64, t477: f64, t822: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t19430 = t1423 * t7551;
    let t19431 = 2.0_f64 / 27.0_f64 * t19430;
    let t19433 = t132 * t435 * t7735;
    let t19434 = t19433 / 15.0_f64;
    let t19435 = t1447 * t7559;
    let t19436 = 4.0_f64 / 45.0_f64 * t19435;
    let t19437 = t1423 * t7563;
    let t19438 = 4.0_f64 / 45.0_f64 * t19437;
    let t19439 = t1447 * t7517;
    let t19440 = 2.0_f64 / 27.0_f64 * t19439;
    let t19441 = 4.0_f64 / 15.0_f64 * t15739;
    let t19442 = 4.0_f64 / 135.0_f64 * t11864;
    let t19447 = 3.0_f64 / 5.0_f64 * t439 * t13788 * t2648 * t822 * t477;
    (t19431, t19434, t19436, t19438, t19440, t19441, t19442, t19447)
}
