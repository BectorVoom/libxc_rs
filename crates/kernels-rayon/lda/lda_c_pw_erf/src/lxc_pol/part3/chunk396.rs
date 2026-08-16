//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 396/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk396(t1458: f64, t22: f64, t1245: f64, t197: f64, t940: f64, t519: f64, t1416: f64, t1420: f64, t1424: f64, t1429: f64, t1435: f64, t1436: f64, t1439: f64, t1445: f64, t1448: f64, t1452: f64, t1456: f64, t256: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1459 = t22 * t1458;
    let t1460 = t197 * t1245;
    let t1461 = t1460 * t940;
    let t1462 = t1459 * t1461;
    let t1464 = 4.0_f64 / 27.0_f64 * t519 * t1462;
    let t1465 = t1416 * t256 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t1420 + 0.12155555555555556_f64 * t1424 + t1429 + t1435 + 4.0_f64 / 9.0_f64 * t1436 + t1439 - t1445 + t1448 + t1452 + t1456 + t1464;
    (t1459, t1460, t1461, t1462, t1464, t1465)
}
