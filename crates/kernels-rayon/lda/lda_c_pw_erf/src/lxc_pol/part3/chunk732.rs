//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 732/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk732(t173: f64, t4645: f64, t184: f64, t199: f64, t4206: f64, t4209: f64, t4563: f64, t4566: f64, t4570: f64, t4573: f64, t4578: f64, t4580: f64, t4583: f64, t4584: f64, t4585: f64, t4586: f64, t4587: f64, t4591: f64, t4593: f64, t4595: f64) -> (f64, f64, f64, f64) {
    let t4646 = t173 * t4645;
    let t4647 = t4646 * t184;
    let t4649 = 2.0_f64 / 15.0_f64 * t4647 * t199;
    let t4650 = t4206 - t4209 + t4563 - t4566 - t4570 - t4573 - t4578 - t4580 + t4583 - t4584 + t4585 + t4586 - t4587 + t4591 - t4593 - t4595 + t4649;
    (t4646, t4647, t4649, t4650)
}
