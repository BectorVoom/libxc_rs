//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 930/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk930(t153: f64, t274: f64, t8798: f64, t242: f64, t4130: f64, t1155: f64, t632: f64, t4137: f64, t1198: f64, t1426: f64, t1159: f64, t646: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11006 = 19.1926369973667_f64 * t153 * t8798 * t274;
    let t11007 = t4130 * t242;
    let t11010 = 2.0103076928521055_f64 * t1155 * t632;
    let t11012 = 2.0103076928521055_f64 * t4137 * t242;
    let t11020 = t1198 * t1426;
    let t11022 = t1159 * t646;
    (t11006, t11007, t11010, t11012, t11020, t11022)
}
