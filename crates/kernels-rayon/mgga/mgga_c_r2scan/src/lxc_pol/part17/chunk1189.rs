//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1189/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1189(t30057: f64, t3308: f64, t6218: f64, t11711: f64, t8240: f64, t11714: f64, t7383: f64, t10856: f64, t9377: f64, t37769: f64, t9373: f64, t3309: f64, t9327: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t43575 = t6218 * t3308 * t30057;
    let t43577 = t8240 * t11711;
    let t43579 = t7383 * t11714;
    let t43581 = t10856 * t9377;
    let t43586 = t37769 * t9373;
    let t43588 = t9327 * t3309;
    (t43575, t43577, t43579, t43581, t43586, t43588)
}
