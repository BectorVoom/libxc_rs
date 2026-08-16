//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1018/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1018(t322: f64, t12240: f64, t3730: f64, t833: f64, t3735: f64, t829: f64, t1013: f64, t3506: f64, t1120: f64, t2394: f64, t11220: f64, t11223: f64, t1300: f64, t2400: f64, t327: f64, t3509: f64, t6693: f64, t834: f64) -> (f64, f64, f64, f64, f64) {
    let t324 = 0.0_f64 < t322;
    let t12241 = piecewise3(t324, 0.0_f64, t12240);
    let t12244 = t3730 * t833;
    let t12253 = t3735 * t829;
    let t12256 = t3506 * t1013;
    let t12259 = t1120 * t2394;
    let t12262 = t3730 * t829;
    let t12267 = -0.64e0_f64 * t12241 * t327 - 0.128e1_f64 * t12244 * t829 - 0.128e1_f64 * t11220 * t1013 - 0.384e1_f64 * t11223 * t2400 - 0.128e1_f64 * t3509 * t2394 - 0.384e1_f64 * t6693 * t12253 - 0.128e1_f64 * t1300 * t12256 - 0.128e1_f64 * t1300 * t12259 - 0.128e1_f64 * t1300 * t12262 - 0.64e0_f64 * t834 * t12241;
    (t12241, t12244, t12256, t12259, t12267)
}
