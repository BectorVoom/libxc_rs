//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 816/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk816(t133: f64, t5506: f64, t5521: f64, t3280: f64, t3284: f64, t3322: f64, t3348: f64, t3361: f64, t5550: f64, t5570: f64, t5577: f64, t5588: f64, t5591: f64, t5609: f64) -> f64 {
    let t5660 = t133 * t5506;
    let t5663 = 1.1495033333333333_f64 * t133 * t5521;
    let t5666 = -1.724255_f64 * t3361 + t3280 - t3284 - t5570 - t3348 - t5577 + t5588 + t5591 - 0.7663355555555555_f64 * t5660 + t5663 - 1.724255_f64 * t133 * t5550 - t5609 - t3322;
    t5666
}
