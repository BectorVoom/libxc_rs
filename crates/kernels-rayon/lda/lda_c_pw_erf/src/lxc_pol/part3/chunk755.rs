//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 755/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk755(t4880: f64, t494: f64, t1440: f64, t1325: f64, t1390: f64, t2098: f64, t542: f64, t519: f64, t1476: f64, t2146: f64, t213: f64, t473: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4881 = t4880 * t494;
    let t4882 = t1440 * t4881;
    let t4884 = 8.0_f64 / 15.0_f64 * t1325 * t4882;
    let t4885 = t1390 * t2098;
    let t4886 = t4885 * t542;
    let t4887 = t1440 * t4886;
    let t4889 = 8.0_f64 / 15.0_f64 * t519 * t4887;
    let t4891 = 16.0_f64 / 135.0_f64 * t2146 * t1476;
    let t4892 = t473 * t213;
    (t4881, t4882, t4884, t4885, t4886, t4887, t4889, t4891, t4892)
}
