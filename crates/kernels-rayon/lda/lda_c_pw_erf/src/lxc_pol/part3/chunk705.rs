//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 705/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk705(t39: f64, t780: f64, t159: f64, t285: f64, t1549: f64, t1809: f64, t1729: f64, t776: f64, t2306: f64, t684: f64, t2310: f64, t1738: f64, t872: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4437 = t39 * t780;
    let t4439 = t4437 * t159 * t285;
    let t4441 = t1549 * t1809;
    let t4449 = t1729 * t776;
    let t4454 = 0.039914113367515366_f64 * t684 * t2306;
    let t4455 = t684 * t2310;
    let t4457 = t1738 * t872;
    (t4437, t4439, t4441, t4449, t4454, t4455, t4457)
}
