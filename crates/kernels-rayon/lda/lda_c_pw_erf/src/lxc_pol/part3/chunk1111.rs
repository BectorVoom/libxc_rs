//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1111/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1111(t10162: f64, t1325: f64, t2182: f64, t2188: f64, t3745: f64, t10169: f64, t10172: f64, t10173: f64, t12971: f64, t12975: f64, t12979: f64, t12982: f64, t12985: f64, t12988: f64, t12991: f64, t12996: f64) -> (f64, f64, f64) {
    let t12998 = t1325 * t10162 * t2182;
    let t12999 = 8.0_f64 / 45.0_f64 * t12998;
    let t13001 = 8.0_f64 / 5.0_f64 * t3745 * t2188;
    let t13002 = t12971 + 8.0_f64 * t10169 - t10172 + 4.0_f64 / 3.0_f64 * t10173 - t12975 + t12979 - t12982 + t12985 + t12988 + t12991 - t12996 + t12999 + t13001;
    (t12999, t13001, t13002)
}
