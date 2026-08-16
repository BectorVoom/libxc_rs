//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1358/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1358(t10712: f64, t10715: f64, t10718: f64, t10719: f64, t14314: f64, t14352: f64, t14366: f64, t23071: f64, t23073: f64, t23076: f64, t23077: f64, t23078: f64, t23081: f64, t23083: f64) -> f64 {
    let t23342 = t10712 - t10715 + t10718 - 0.011181742741110338_f64 * t10719 + t23071 - t23073 + t14314 - t14352 - t23076 - t23077 + t23078 - t23081 - t23083 + t14366;
    t23342
}
