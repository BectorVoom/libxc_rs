//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1178/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1178(t3308: f64, t6449: f64, t8807: f64, t10776: f64, t8826: f64, t3295: f64, t9160: f64, t9156: f64, t10781: f64, t8813: f64, t11802: f64, t39375: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t43083 = t6449 * t3308 * t8807;
    let t43086 = t10776 * t3308 * t8826;
    let t43088 = t3295 * t9160;
    let t43090 = t3295 * t9156;
    let t43092 = t10781 * t8813;
    let t43094 = t39375 * t11802;
    (t43083, t43086, t43088, t43090, t43092, t43094)
}
