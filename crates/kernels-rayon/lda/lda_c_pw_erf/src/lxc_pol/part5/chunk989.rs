//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 989/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk989(t1143: f64, t1905: f64, t2929: f64, t781: f64, t242: f64, t4422: f64, t4437: f64, t5446: f64, t646: f64, t1426: f64, t1901: f64, t1896: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14956 = t1905 * t1143;
    let t14957 = 0.2512884616065132_f64 * t14956;
    let t14958 = t781 * t2929;
    let t14960 = t4422 * t242;
    let t14961 = 0.5025769232130264_f64 * t14960;
    let t14965 = t4437 * t242;
    let t14978 = t5446 * t646;
    let t14979 = 0.09973633333333333_f64 * t14978;
    let t14980 = t1901 * t1426;
    let t14992 = t1896 * t646;
    (t14957, t14958, t14961, t14965, t14979, t14980, t14992)
}
