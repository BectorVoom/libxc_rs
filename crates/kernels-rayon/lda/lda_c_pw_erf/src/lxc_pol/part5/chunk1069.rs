//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1069/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1069(t11325: f64, t11328: f64, t11333: f64, t11338: f64, t11340: f64, t11341: f64, t11342: f64, t11343: f64, t11344: f64, t15413: f64, t15421: f64, t19987: f64, t20035: f64, t20037: f64, t20039: f64, t20041: f64, t20043: f64, t20044: f64, t8202: f64) -> f64 {
    let t20046 = -t19987 - 5.476843845342223_f64 * t11325 + t11328 + t20035 + t20037 + t20039 - t20041 - 1.232289865202_f64 * t15413 + t11333 - t20043 - t20044 + t11338 + t11340 + 2.0538164420033334_f64 * t15421 - t11341 - t11342 - t8202 - t11343 - t11344;
    t20046
}
