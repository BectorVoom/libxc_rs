//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 883/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk883(t1063: f64, t147: f64, t159: f64, t285: f64, t142: f64, t3363: f64, t454: f64, t1553: f64, t1726: f64, t405: f64, t2863: f64, t684: f64) -> (f64, f64, f64, f64, f64) {
    let t8756 = t1063 * t147;
    let t8759 = 0.03831185177913979_f64 * t8756 * t159 * t285;
    let t8761 = t454 * t3363 * t142;
    let t8768 = t405 * t1726 * t1553;
    let t8771 = t684 * t2863;
    (t8756, t8759, t8761, t8768, t8771)
}
