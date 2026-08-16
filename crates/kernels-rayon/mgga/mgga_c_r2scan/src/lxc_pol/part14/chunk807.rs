//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 807/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk807(t1048: f64, t2262: f64, t2867: f64, t1234: f64, t2859: f64, t2858: f64, t481: f64, t795: f64, t2266: f64, t5027: f64, t5029: f64, t4703: f64, t4721: f64, t4880: f64, t4891: f64, t4901: f64, t4964: f64, t4967: f64, t6943: f64, t6946: f64, t6947: f64, t6948: f64, t6949: f64, t6950: f64, t6951: f64, t6952: f64, t6954: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7141 = t1048 * t2867 * t2262;
    let t7142 = t2859 * t1234;
    let t7143 = t2858 * t7142;
    let t7144 = 6.0_f64 * t7143;
    let t7145 = t481 * t795;
    let t7147 = t2266 * t2867 * t7145;
    let t7148 = 6.0_f64 * t7147;
    let t7149 = 16.0_f64 * t5027;
    let t7150 = 0.11696447245269292414e1_f64 * t5029;
    let t7151 = t6943 + t4880 - t6946 + t6947 + t6948 - t4891 - t6949 - t6950 + t4703 - t6951 - t6952 + t4901 + t4721 - t4964 + t4967 + t6954;
    (t7141, t7144, t7148, t7149, t7150, t7151)
}
