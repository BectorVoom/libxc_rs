//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1181/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1181(t15012: f64, t4581: f64, t13475: f64, t5142: f64, t13462: f64, t5134: f64, t339: f64, t9368: f64, t13467: f64, t13516: f64, t1045: f64, t934: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15013 = t15012 * t4581;
    let t15016 = t5142 * t13475;
    let t15019 = t5134 * t13462;
    let t15022 = t9368 * t339;
    let t15023 = t15022 * t13467;
    let t15026 = t5134 * t13516;
    let t15036 = t934 * t1045;
    (t15013, t15016, t15019, t15023, t15026, t15036)
}
