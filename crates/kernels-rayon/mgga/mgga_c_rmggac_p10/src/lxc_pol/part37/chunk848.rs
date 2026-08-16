//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 848/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk848(t14173: f64, t1652: f64, t1550: f64, t16503: f64, t35039: f64, t665: f64, t8420: f64, t16504: f64, t8425: f64, t3369: f64, t8430: f64, t2024: f64, t34976: f64, t8435: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t75216 = t14173 * t1652;
    let t75217 = t1550 * t75216;
    let t75221 = t16503 * t35039 * t665 * t8420;
    let t75225 = t16503 * t16504 * t665 * t8425;
    let t75231 = t16503 * t3369 * t665 * t8430;
    let t75235 = t16503 * t34976 * t2024 * t8435;
    (t75216, t75217, t75221, t75225, t75231, t75235)
}
