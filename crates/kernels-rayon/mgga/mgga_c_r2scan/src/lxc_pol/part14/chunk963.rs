//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 963/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk963(t322: f64, t11216: f64, t3506: f64, t833: f64, t1120: f64, t1299: f64, t1295: f64, t829: f64, t1292: f64, t1300: f64, t327: f64, t3509: f64, t6693: f64, t834: f64) -> (f64, f64, f64, f64) {
    let t324 = 0.0_f64 < t322;
    let t11217 = piecewise3(t324, 0.0_f64, t11216);
    let t11220 = t3506 * t833;
    let t11223 = t1120 * t1299;
    let t11228 = t1120 * t1295;
    let t11231 = t3506 * t829;
    let t11234 = t1120 * t1292;
    let t11239 = -0.64e0_f64 * t11217 * t327 - 0.256e1_f64 * t11220 * t829 - 0.384e1_f64 * t11223 * t1295 - 0.128e1_f64 * t3509 * t1292 - 0.384e1_f64 * t6693 * t11228 - 0.256e1_f64 * t1300 * t11231 - 0.128e1_f64 * t1300 * t11234 - 0.64e0_f64 * t834 * t11217;
    (t11217, t11220, t11223, t11239)
}
