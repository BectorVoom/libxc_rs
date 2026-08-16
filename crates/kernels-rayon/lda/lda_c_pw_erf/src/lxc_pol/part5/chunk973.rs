//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 973/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk973(t4521: f64, t811: f64, t34: f64, t3975: f64, t2070: f64, t807: f64, t185: f64, t834: f64, t211: f64, t548: f64, t812: f64, t4039: f64, t795: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14030 = t4521 * t811;
    let t14034 = t3975 * t34;
    let t14043 = t2070 * t807;
    let t14044 = t185 * t14043;
    let t14048 = t2070 * t834;
    let t14049 = t211 * t14048;
    let t14052 = t548 * t2070 * t812;
    let t14089 = t795 * t4039;
    (t14030, t14034, t14044, t14049, t14052, t14089)
}
