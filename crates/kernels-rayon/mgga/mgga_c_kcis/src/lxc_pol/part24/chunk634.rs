//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 634/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk634(t1260: f64, t6837: f64, t286: f64, t1251: f64, t3499: f64, t3514: f64, t5300: f64, t5322: f64, t6759: f64, t6763: f64, t6767: f64, t6771: f64, t6776: f64) -> (f64, f64, f64) {
    let t6838 = t1260 * t6837;
    let t6839 = t286 * t6838;
    let t6842 = -t3499 + t5300 / 864.0_f64 - t5322 / 288.0_f64 + t1251 * t6759 / 432.0_f64 - t3514 * t6763 / 288.0_f64 - t1251 * t6767 / 288.0_f64 + t1251 * t6771 / 576.0_f64 + t1251 * t6776 / 96.0_f64 - t1251 * t6839 / 192.0_f64;
    (t6838, t6839, t6842)
}
