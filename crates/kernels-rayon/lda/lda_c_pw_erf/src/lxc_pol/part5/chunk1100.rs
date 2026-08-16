//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1100/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1100(t14582: f64, t19739: f64, t19750: f64, t19753: f64, t20305: f64, t20318: f64, t20319: f64, t20324: f64, t20328: f64, t20329: f64, t20330: f64, t8936: f64, t9096: f64) -> f64 {
    let t20513 = t20305 - t8936 + t20318 - t20319 - t20324 - t20328 + t20329 - 5.172765_f64 * t19739 + 20.69106_f64 * t19750 - 10.34553_f64 * t19753 + t20330 + 1.7881162962962962_f64 * t9096 + 5.364348888888889_f64 * t14582;
    t20513
}
