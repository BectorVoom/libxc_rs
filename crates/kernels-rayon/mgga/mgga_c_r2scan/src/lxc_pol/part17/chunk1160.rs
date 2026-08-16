//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1160/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1160(t3295: f64, t9156: f64, t10781: f64, t8813: f64, t11802: f64, t39375: f64, t10710: f64, t30428: f64, t37712: f64, t10768: f64, t29194: f64, t29177: f64, t37658: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t43090 = t3295 * t9156;
    let t43092 = t10781 * t8813;
    let t43094 = t39375 * t11802;
    let t43097 = t37712 * t10710 * t30428;
    let t43100 = t10768 * t10710 * t29194;
    let t43103 = t37658 * t10710 * t29177;
    (t43090, t43092, t43094, t43097, t43100, t43103)
}
