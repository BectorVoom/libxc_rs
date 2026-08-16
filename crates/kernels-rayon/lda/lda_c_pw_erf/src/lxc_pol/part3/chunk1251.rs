//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1251/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1251(t2765: f64, t2777: f64, t756: f64, t281: f64, t285: f64, t4713: f64, t477: f64, t1128: f64, t1904: f64, t2872: f64, t780: f64, t1184: f64, t1187: f64, t483: f64) -> (f64, f64, f64, f64, f64) {
    let t14876 = t2765 * t756 * t2777;
    let t14891 = t281 * t4713 * t477 * t285;
    let t14895 = t281 * t1904 * t1128 * t285;
    let t14896 = 0.03592270203076383_f64 * t14895;
    let t14899 = t281 * t780 * t2872 * t285;
    let t14903 = t1184 * t1904 * t483 * t1187;
    (t14876, t14891, t14896, t14899, t14903)
}
