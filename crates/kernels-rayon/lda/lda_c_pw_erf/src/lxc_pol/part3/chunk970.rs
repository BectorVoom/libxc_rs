//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 970/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk970(t1143: f64, t1198: f64, t2929: f64, t458: f64, t116: f64, t1191: f64, t731: f64, t732: f64, t2693: f64, t2695: f64, t726: f64, t4291: f64, t4299: f64) -> (f64, f64, f64, f64, f64) {
    let t11233 = t1198 * t1143;
    let t11236 = 0.3350512821420176_f64 * t458 * t2929;
    let t11250 = 6.693920255418272_f64 * t731 * t732 * t1191 * t116;
    let t11254 = t726 * t2693 * t2695;
    let t11256 = t4291 * t4299;
    (t11233, t11236, t11250, t11254, t11256)
}
