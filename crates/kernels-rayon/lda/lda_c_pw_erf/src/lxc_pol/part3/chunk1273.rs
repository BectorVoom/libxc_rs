//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1273/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1273(t11035: f64, t11038: f64, t12503: f64, t12504: f64, t12505: f64, t12520: f64, t12523: f64, t12524: f64, t12525: f64, t12528: f64, t12530: f64, t12533: f64, t12535: f64) -> f64 {
    let t15019 = t12503 + t12504 + t12505 - t12520 + t12523 + 0.09973633333333333_f64 * t11035 + t12524 - t12525 - t11038 - t12528 - t12530 - t12533 - t12535;
    t15019
}
