//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1029/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1029(t238: f64, t5158: f64, t86: f64, t284: f64, t5082: f64, t10506: f64, t251: f64, t1281: f64, t5358: f64, t13101: f64, t13103: f64, t1844: f64, t3643: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t15046 = 0.53062222222222222222e-1_f64 * t86 * t238 * t5158;
    let t15068 = t5082 * t284;
    let t15085 = t251 * t10506;
    let t15109 = t5358 * t1281;
    let t15112 = 0.23214722222222222222e-2_f64 * t13101;
    let t15113 = 0.15476481481481481481e-2_f64 * t13103;
    let t15134 = t1844 * t3643;
    (t15046, t15068, t15085, t15109, t15112, t15113, t15134)
}
