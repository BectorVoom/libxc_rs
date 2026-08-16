//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1070/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1070(t12536: f64, t525: f64, t11636: f64, t12508: f64, t12509: f64, t12514: f64, t12520: f64, t12523: f64, t12524: f64, t12525: f64, t12528: f64, t12530: f64, t12533: f64, t12535: f64, t225: f64, t231: f64) -> (f64, f64) {
    let t12538 = 4.0_f64 / 15.0_f64 * t12536 * t525;
    let t12539 = t12508 + 4.0_f64 / 3.0_f64 * t12509 + 4.0_f64 / 3.0_f64 * t11636 * t225 * t231 + 4.0_f64 * t12514 - t12520 + t12523 + t12524 - t12525 - t12528 - t12530 - t12533 - t12535 + t12538;
    (t12538, t12539)
}
