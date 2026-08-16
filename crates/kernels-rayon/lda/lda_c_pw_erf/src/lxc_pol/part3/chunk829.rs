//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 829/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk829(t5795: f64, t647: f64, t1432: f64, t850: f64, t256: f64, t1427: f64, t2260: f64, t4464: f64, t4466: f64, t4468: f64, t4470: f64, t4471: f64, t4472: f64, t4473: f64, t4474: f64, t4478: f64, t4482: f64, t4486: f64, t5788: f64, t5793: f64) -> (f64, f64) {
    let t5797 = 0.12155555555555556_f64 * t5795 * t647;
    let t5798 = t850 * t1432;
    let t5799 = t5798 * t256;
    let t5801 = t2260 * t1427;
    let t5803 = t5788 * t256 / 3.0_f64 + t5793 + t5797 + t5799 / 3.0_f64 + 0.12155555555555556_f64 * t5801 - t4464 - t4466 + t4468 + t4470 + t4471 + t4472 + t4473 + t4474 - t4478 - t4482 - t4486;
    (t5798, t5803)
}
