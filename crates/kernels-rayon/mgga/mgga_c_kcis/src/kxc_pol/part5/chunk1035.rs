//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1035/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1035(t1251: f64, t15554: f64, t25: f64, t287: f64, t5331: f64, t13391: f64, t13408: f64, t14078: f64, t14081: f64, t14085: f64, t14104: f64, t14567: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15555 = t1251 * t15554;
    let t15573 = t25 * t287;
    let t15574 = t15573 * t5331;
    let t15576 = t1251 * t15574 / 144.0_f64;
    let t15602 = 0.15476481481481481481e-2_f64 * t13391;
    let t15607 = 0.15476481481481481481e-2_f64 * t13408;
    let t15632 = 0.23214722222222222222e-2_f64 * t14078;
    let t15638 = 0.30952962962962962962e-2_f64 * t14081;
    let t15639 = 0.15476481481481481481e-2_f64 * t14085;
    let t15648 = 0.15476481481481481481e-2_f64 * t14104;
    let t15659 = 0.23214722222222222222e-2_f64 * t14567;
    (t15555, t15576, t15602, t15607, t15632, t15638, t15639, t15648, t15659)
}
