//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 987/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk987(t3709: f64, t682: f64, t696: f64, t8599: f64, t1025: f64, t1035: f64, t3666: f64, t3669: f64, t3952: f64, t687: f64, t3947: f64, t654: f64) -> (f64, f64, f64, f64) {
    let t8603 = 14.03573669432315_f64 * t696 * t3709 * t8599 * t682;
    let t8610 = 3103.560775156404_f64 * t3666 * t1035 * t3669 * t1025;
    let t8612 = 480.0_f64 * t3952 * t687;
    let t8614 = t3947 * t654;
    (t8603, t8610, t8612, t8614)
}
