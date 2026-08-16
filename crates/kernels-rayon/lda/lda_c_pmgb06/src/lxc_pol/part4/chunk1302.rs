//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1302/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1302(t15223: f64, t36: f64, t506: f64, t1476: f64, t16003: f64, t16359: f64, t2909: f64, t103: f64, t13382: f64, t14110: f64, t14150: f64, t15200: f64, t15548: f64, t17070: f64, t525: f64, t9577: f64, t9956: f64, t9958: f64, t9981: f64, t9986: f64) -> (f64, f64, f64, f64) {
    let t17107 = t36 * t506 * t15223;
    let t17110 = t36 * t1476 * t16003;
    let t17113 = t36 * t2909 * t16359;
    let t17121 = -0.007407407407407408_f64 * t9956 - 0.0024691358024691358_f64 * t9958 - 0.047988888888888886_f64 * t13382 + t9981 + 0.03199259259259259_f64 * t9577 + t9986 + 0.023703703703703703_f64 * t15548 * t14110 * t17070 - 0.017777777777777778_f64 * t14150 + 0.8638_f64 * t17107 - 0.8638_f64 * t17110 + 0.47988888888888886_f64 * t17113 - 0.04_f64 * t103 * t525 * t15200 + 0.16_f64 * t103 * t525 * t15223;
    (t17107, t17110, t17113, t17121)
}
