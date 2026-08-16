//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1200/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1200(t12439: f64, t12441: f64, t12446: f64, t12448: f64, t12450: f64, t12452: f64, t12454: f64, t12457: f64, t12459: f64, t12461: f64, t12463: f64, t12466: f64, t12469: f64, t12473: f64, t12476: f64, t12479: f64, t12484: f64, t12488: f64, t12491: f64, t12493: f64, t12496: f64, t12500: f64, t12504: f64) -> (f64, f64) {
    let t14369 = t12439 + t12441 + t12446 - t12448 - t12450 + t12452 + t12454 - t12457 - t12459 - t12461 - t12463;
    let t14370 = -t12466 + t12469 + t12473 - t12476 + t12479 + t12484 + t12488 - t12491 + t12493 - t12496 + t12500 + t12504;
    (t14369, t14370)
}
