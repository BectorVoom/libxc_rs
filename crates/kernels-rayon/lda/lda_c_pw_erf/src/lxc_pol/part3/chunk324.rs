//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 324/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk324(t1084: f64, t1085: f64, t1001: f64, t1061: f64, t1066: f64, t1069: f64, t1072: f64, t1075: f64, t1079: f64, t1083: f64, t910: f64, t938: f64, t997: f64) -> (f64, f64) {
    let t1086 = t1084 * t1085;
    let t1087 = 0.010843580882781523_f64 * t1086;
    let t1088 = -t1061 + t1066 + t1069 - t1072 - t997 + t938 + t910 - t1001 - t1075 + t1079 + t1083 + t1087;
    (t1087, t1088)
}
