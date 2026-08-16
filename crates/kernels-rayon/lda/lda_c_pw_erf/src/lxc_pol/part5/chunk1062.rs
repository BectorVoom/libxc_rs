//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1062/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1062(t11307: f64, t11309: f64, t1: f64, t397: f64, t7376: f64, t8180: f64, t11313: f64, t11315: f64, t11317: f64, t11319: f64, t15341: f64, t15344: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t19972 = 10.526802115419367_f64 * t11307;
    let t19973 = 155.84180309438278_f64 * t11309;
    let t19975 = t7376 * t1 * t397;
    let t19976 = 0.0001831155503675316_f64 * t19975;
    let t19977 = 1025.3897021007795_f64 * t8180;
    let t19978 = 0.06506148529668915_f64 * t11313;
    let t19979 = 0.09759222794503372_f64 * t11315;
    let t19980 = 0.04879611397251686_f64 * t11317;
    let t19981 = 1.4447833828541736_f64 * t11319;
    let t19982 = 51.94726769812759_f64 * t15341;
    let t19983 = 1.7544670192365612_f64 * t15344;
    (t19972, t19973, t19976, t19977, t19978, t19979, t19980, t19981, t19982, t19983)
}
