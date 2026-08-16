//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1240/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1240(t1325: f64, t34: f64, t4956: f64, t6944: f64, t10557: f64, t519: f64, t7624: f64, t1449: f64, t7620: f64, t2171: f64, t6699: f64, t22263: f64, t22266: f64, t22268: f64, t22272: f64, t22276: f64, t22280: f64, t22284: f64, t22288: f64, t22292: f64) -> (f64, f64, f64, f64, f64) {
    let t22296 = 8.0_f64 / 5.0_f64 * t1325 * t4956 * t6944 * t34;
    let t22298 = t519 * t10557 * t7624;
    let t22299 = 64.0_f64 / 243.0_f64 * t22298;
    let t22301 = t519 * t1449 * t7620;
    let t22302 = 8.0_f64 / 135.0_f64 * t22301;
    let t22303 = t2171 * t6699;
    let t22304 = 8.0_f64 / 27.0_f64 * t22303;
    let t22305 = t22263 + t22266 - t22268 + t22272 - t22276 - t22280 + t22284 + t22288 + t22292 + t22296 + t22299 + t22302 + t22304;
    (t22296, t22299, t22302, t22304, t22305)
}
