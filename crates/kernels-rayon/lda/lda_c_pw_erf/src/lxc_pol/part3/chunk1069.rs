//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1069/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1069(t9593: f64, t9596: f64, t1318: f64, t3899: f64, t5355: f64, t3416: f64, t4933: f64, t5316: f64, t2158: f64, t9752: f64, t4646: f64, t518: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12524 = 32.0_f64 / 135.0_f64 * t9593;
    let t12525 = 8.0_f64 / 45.0_f64 * t9596;
    let t12527 = t1318 * t3899 * t5355;
    let t12528 = 8.0_f64 / 15.0_f64 * t12527;
    let t12529 = t3416 * t4933;
    let t12530 = 16.0_f64 / 15.0_f64 * t12529;
    let t12532 = t1318 * t3899 * t5316;
    let t12533 = 16.0_f64 / 15.0_f64 * t12532;
    let t12535 = 4.0_f64 / 5.0_f64 * t9752 * t2158;
    let t12536 = t4646 * t518;
    (t12524, t12525, t12528, t12530, t12533, t12535, t12536)
}
