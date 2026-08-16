//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 469/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk469(t1409: f64, t1412: f64, t1420: f64, t1424: f64, t1429: f64, t1435: f64, t1436: f64, t1439: f64, t2009: f64, t2013: f64, t2016: f64, t2020: f64, t2025: f64, t2029: f64, t2033: f64, t2037: f64, t2039: f64, t2266: f64, t2281: f64, t267: f64) -> f64 {
    let t2287 = t2009 - t2013 - t2016 + t2020 - t2025 + t2029 - t2033 + t2037 - 2.0_f64 / 45.0_f64 * t2266 - t2281 * t267 / 15.0_f64 + t1409 - t1412 + t1420 / 3.0_f64 + 0.06077777777777778_f64 * t1424 + t1429 + t1435 + 2.0_f64 / 9.0_f64 * t1436 + t1439 + t2039;
    t2287
}
