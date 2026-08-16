//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1279/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1279(t11046: f64, t11050: f64, t11053: f64, t11055: f64, t11057: f64, t11063: f64, t11065: f64, t11069: f64, t11073: f64, t11074: f64, t11079: f64, t11081: f64, t11088: f64, t12718: f64) -> f64 {
    let t15035 = t11046 / 3.0_f64 + 0.06077777777777778_f64 * t11050 + t11053 + 0.36466666666666664_f64 * t11055 + 2.0_f64 * t11057 + t11063 + 0.004546314527777778_f64 * t11065 + t11069 + t11073 + 0.547_f64 * t11074 + t11079 + t11081 + t11088 - t12718;
    t15035
}
