//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 721/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk721(t4517: f64, t4522: f64, t4506: f64, t3443: f64, t3446: f64, t3458: f64, t3551: f64, t3554: f64, t3557: f64, t3570: f64, t3577: f64, t3661: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4523 = t4522 * t4517;
    let t4525 = 8.0_f64 / 27.0_f64 * t4506 * t4523;
    let t4526 = 16.0_f64 / 45.0_f64 * t3443;
    let t4527 = 8.0_f64 / 45.0_f64 * t3446;
    let t4528 = 8.0_f64 / 45.0_f64 * t3458;
    let t4529 = 16.0_f64 / 135.0_f64 * t3551;
    let t4530 = 8.0_f64 / 135.0_f64 * t3554;
    let t4531 = 4.0_f64 / 45.0_f64 * t3557;
    let t4532 = 8.0_f64 / 45.0_f64 * t3570;
    let t4533 = 4.0_f64 / 45.0_f64 * t3577;
    let t4534 = 16.0_f64 / 135.0_f64 * t3661;
    (t4523, t4525, t4526, t4527, t4528, t4529, t4530, t4531, t4532, t4533, t4534)
}
