//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 495/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk495(t2010: f64, t2012: f64, t435: f64, t823: f64, t132: f64, t489: f64, t852: f64, t161: f64, t1798: f64, t205: f64, t208: f64, t579: f64, t794: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2014 = 2.0_f64 / 45.0_f64 * t2010 * t2012;
    let t2015 = t435 * t823;
    let t2016 = t132 * t2015;
    let t2017 = t2016 / 45.0_f64;
    let t2018 = t489 * t852;
    let t2019 = t161 * t2018;
    let t2020 = t2019 / 45.0_f64;
    let t2021 = t1798 * t205;
    let t2022 = t2021 * t208;
    let t2025 = t794 * t579;
    (t2014, t2015, t2017, t2018, t2020, t2021, t2022, t2025)
}
