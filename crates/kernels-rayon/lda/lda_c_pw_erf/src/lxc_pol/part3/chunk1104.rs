//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1104/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1104(t3775: f64, t4738: f64, t1124: f64, t213: f64, t1318: f64, t4894: f64, t1381: f64, t4892: f64, t4893: f64, t2146: f64, t4063: f64, t4044: f64, t4763: f64) -> (f64, f64, f64, f64, f64) {
    let t12915 = 4.0_f64 / 5.0_f64 * t4738 * t3775;
    let t12916 = t1124 * t213;
    let t12918 = t1318 * t12916 * t4894;
    let t12919 = 4.0_f64 / 3.0_f64 * t12918;
    let t12923 = 4.0_f64 / 5.0_f64 * t1318 * t4892 * t4893 * t1381;
    let t12924 = t2146 * t4063;
    let t12925 = 8.0_f64 / 27.0_f64 * t12924;
    let t12927 = 8.0_f64 / 5.0_f64 * t4763 * t4044;
    (t12915, t12919, t12923, t12925, t12927)
}
