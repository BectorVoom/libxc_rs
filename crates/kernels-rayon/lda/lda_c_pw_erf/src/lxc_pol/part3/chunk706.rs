//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 706/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk706(t1733: f64, t2211: f64, t2764: f64, t2772: f64, t2779: f64, t2799: f64, t2801: f64, t2811: f64, t4425: f64, t4427: f64, t4430: f64, t4435: f64, t4439: f64, t4441: f64, t4449: f64, t4454: f64, t4455: f64, t4457: f64, t777: f64) -> f64 {
    let t4459 = -t4425 - 0.0005811348303577384_f64 * t4427 - 3.0_f64 * t2764 * t4430 + 0.19816831758676853_f64 * t4435 + 0.001355981270834723_f64 * t4439 + 3.0_f64 * t1733 * t4441 - t777 * t2799 + 2.0_f64 * t777 * t2779 + 3.0_f64 * t2211 * t2801 + 6.0_f64 * t4449 * t2811 - 0.054045904796391424_f64 * t2772 + t4454 + 0.039914113367515366_f64 * t4455 - 0.05321881782335382_f64 * t4457;
    t4459
}
