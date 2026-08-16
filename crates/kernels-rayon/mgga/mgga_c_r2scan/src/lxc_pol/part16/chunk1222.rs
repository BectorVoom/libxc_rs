//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1222/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1222(t37769: f64, t9373: f64, t3309: f64, t9327: f64, t2147: f64, t29936: f64, t3332: f64, t11683: f64, t26088: f64, t10760: f64, t29946: f64, t6535: f64) -> (f64, f64, f64, f64, f64) {
    let t43586 = t37769 * t9373;
    let t43588 = t9327 * t3309;
    let t43592 = t2147 * t3332 * t29936;
    let t43594 = t26088 * t11683;
    let t43597 = t6535 * t10760 * t29946;
    (t43586, t43588, t43592, t43594, t43597)
}
