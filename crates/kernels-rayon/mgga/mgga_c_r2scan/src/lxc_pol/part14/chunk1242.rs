//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1242/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1242(t322: f64, t41875: f64, t41890: f64, t12241: f64, t833: f64, t1299: f64, t3730: f64, t1013: f64, t11220: f64, t12244: f64, t1292: f64, t1295: f64, t1300: f64, t2394: f64, t327: f64, t3509: f64, t38834: f64, t6693: f64, t829: f64, t834: f64, t8398: f64) -> (f64, f64) {
    let t324 = 0.0_f64 < t322;
    let t41891 = t41875 + t41890;
    let t41892 = piecewise3(t324, 0.0_f64, t41891);
    let t41901 = t12241 * t833;
    let t41906 = t3730 * t1299;
    let t41917 = -0.256e1_f64 * t1300 * t12241 * t829 - 0.64e0_f64 * t41892 * t327 - 0.128e1_f64 * t1300 * t3730 * t1292 - 0.384e1_f64 * t6693 * t3730 * t1295 - 0.256e1_f64 * t41901 * t829 - 0.128e1_f64 * t12244 * t1292 - 0.384e1_f64 * t41906 * t1295 - 0.128e1_f64 * t38834 * t1013 - 0.256e1_f64 * t11220 * t2394 - 0.128e1_f64 * t3509 * t8398 - 0.64e0_f64 * t834 * t41892;
    (t41891, t41917)
}
