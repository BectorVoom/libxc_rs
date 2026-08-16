//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 810/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk810(t436: f64, t5548: f64, t120: f64, t102: f64, t3296: f64, t756: f64, t1664: f64, t767: f64, t1697: f64, t1832: f64, t1844: f64, t411: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5565 = t436 * t5548;
    let t5568 = t120 * t5548;
    let t5570 = 2.923025_f64 * t102 * t5568;
    let t5571 = t3296 * t756;
    let t5577 = 17.53815_f64 * t102 * t767 * t1664;
    let t5578 = t1697 * t1832;
    let t5588 = 11.6921_f64 * t102 * t1844 * t411;
    (t5565, t5568, t5570, t5571, t5577, t5578, t5588)
}
