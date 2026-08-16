//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 994/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk994(t9178: f64, t9180: f64, t9181: f64, t9186: f64, t9190: f64, t9192: f64, t9195: f64, t9197: f64, t9199: f64, t9201: f64, t9203: f64, t9206: f64, t9207: f64, t9211: f64, t9215: f64) -> f64 {
    let t11619 = t9178 - t9180 - 0.00035595929614954216_f64 * t9181 - 0.031505407223141116_f64 * t9186 - 0.07184540406152766_f64 * t9190 - 0.5670973300165402_f64 * t9192 - t9195 + 0.031505407223141116_f64 * t9197 + 0.1890324433388467_f64 * t9199 - 0.1890324433388467_f64 * t9201 + 0.2835486650082701_f64 * t9203 + t9206 - 0.09451622166942335_f64 * t9207 + 0.2634331482256014_f64 * t9211 + 0.008980675507690957_f64 * t9215;
    t11619
}
