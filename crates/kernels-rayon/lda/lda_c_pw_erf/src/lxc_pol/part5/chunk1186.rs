//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1186/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1186(t17417: f64, t184: f64, t1958: f64, t221: f64, t2400: f64, t21530: f64, t21535: f64, t21540: f64, t21542: f64, t21544: f64, t21549: f64, t21551: f64, t21553: f64, t21554: f64, t21555: f64, t21556: f64) -> (f64, f64, f64) {
    let t21557 = 16.0_f64 / 15.0_f64 * t17417;
    let t21561 = 4.0_f64 / 5.0_f64 * t2400 * t1958 * t184 * t221;
    let t21562 = t21530 + t21535 - t21540 + t21542 - t21544 - t21549 - t21551 - t21553 - t21554 - t21555 - t21556 + t21557 + t21561;
    (t21557, t21561, t21562)
}
