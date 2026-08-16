//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1187/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1187(t10469: f64, t10472: f64, t10500: f64, t10515: f64, t10517: f64, t10529: f64, t10541: f64, t10551: f64, t10559: f64, t10574: f64, t10598: f64, t10603: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13983 = 16.0_f64 / 135.0_f64 * t10469;
    let t13984 = 16.0_f64 / 45.0_f64 * t10472;
    let t13985 = 8.0_f64 / 135.0_f64 * t10500;
    let t13986 = 16.0_f64 / 45.0_f64 * t10515;
    let t13987 = 8.0_f64 / 45.0_f64 * t10517;
    let t13988 = 64.0_f64 / 243.0_f64 * t10529;
    let t13989 = 16.0_f64 / 15.0_f64 * t10541;
    let t13990 = 8.0_f64 / 45.0_f64 * t10551;
    let t13991 = 64.0_f64 / 243.0_f64 * t10559;
    let t13992 = 8.0_f64 / 15.0_f64 * t10574;
    let t13993 = 16.0_f64 / 45.0_f64 * t10598;
    let t13994 = 8.0_f64 / 135.0_f64 * t10603;
    (t13983, t13984, t13985, t13986, t13987, t13988, t13989, t13990, t13991, t13992, t13993, t13994)
}
