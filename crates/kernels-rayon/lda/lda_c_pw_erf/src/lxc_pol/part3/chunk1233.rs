//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1233/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1233(t443: f64, t5616: f64, t102: f64, t1568: f64, t1844: f64, t3251: f64, t763: f64, t1664: f64, t1856: f64, t411: f64, t5549: f64, t1852: f64, t3222: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14535 = t5616 * t443;
    let t14549 = 17.53815_f64 * t102 * t1844 * t1568;
    let t14552 = 5.84605_f64 * t102 * t763 * t3251;
    let t14555 = 52.61445_f64 * t102 * t1856 * t1664;
    let t14558 = 17.53815_f64 * t102 * t5549 * t411;
    let t14561 = 70.1526_f64 * t102 * t1852 * t3222;
    (t14535, t14549, t14552, t14555, t14558, t14561)
}
