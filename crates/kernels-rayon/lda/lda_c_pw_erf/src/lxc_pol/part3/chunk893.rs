//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 893/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk893(t426: f64, t8945: f64, t1250: f64, t47: f64, t1332: f64, t52: f64, t1568: f64, t299: f64, t732: f64, t3257: f64, t1691: f64, t8924: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8946 = t426 * t8945;
    let t8949 = 1.0_f64 / t47 / t1250;
    let t8962 = 1.0_f64 / t52 / t1332;
    let t8980 = t732 * t299 * t1568;
    let t8981 = t3257 * t8980;
    let t8983 = t1691 * t8924;
    (t8946, t8949, t8962, t8980, t8981, t8983)
}
