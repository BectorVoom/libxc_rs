//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 848/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk848(t2407: f64, t808: f64, t2120: f64, t2505: f64, t6209: f64, t7797: f64, t220: f64, t186: f64, t548: f64, t6895: f64, t6897: f64, t5340: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7831 = 4.0_f64 / 5.0_f64 * t2407 * t808;
    let t7833 = 4.0_f64 / 5.0_f64 * t2120 * t2505;
    let t7835 = 4.0_f64 / 5.0_f64 * t6209 * t2505;
    let t7836 = -t7797;
    let t7837 = t220 * t7836;
    let t7838 = t186 * t7837;
    let t7840 = 4.0_f64 / 15.0_f64 * t548 * t7838;
    let t7841 = 16.0_f64 / 15.0_f64 * t6895;
    let t7842 = 16.0_f64 / 45.0_f64 * t6897;
    let t7843 = 8.0_f64 / 135.0_f64 * t5340;
    (t7831, t7833, t7835, t7836, t7837, t7838, t7840, t7841, t7842, t7843)
}
