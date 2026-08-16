//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 840/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk840(t413: f64, t6835: f64, t1260: f64, t286: f64, t1251: f64, t3499: f64, t3514: f64, t5300: f64, t5322: f64, t6759: f64, t6763: f64, t6767: f64, t6771: f64, t6776: f64) -> (f64, f64, f64, f64) {
    let t418 = 0.0_f64 < t413;
    let t6837 = piecewise3(t418, t6835, -t6835);
    let t6838 = t1260 * t6837;
    let t6839 = t286 * t6838;
    let t6842 = -t3499 + t5300 / 864.0_f64 - t5322 / 288.0_f64 + t1251 * t6759 / 432.0_f64 - t3514 * t6763 / 288.0_f64 - t1251 * t6767 / 288.0_f64 + t1251 * t6771 / 576.0_f64 + t1251 * t6776 / 96.0_f64 - t1251 * t6839 / 192.0_f64;
    (t6837, t6838, t6839, t6842)
}
