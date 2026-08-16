//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 974/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk974(t11110: f64, t333: f64, t335: f64, t337: f64, t1083: f64, t1085: f64, t1087: f64, t11092: f64, t11106: f64, t11108: f64, t1310: f64, t3390: f64, t3394: f64, t3398: f64, t839: f64) -> (f64, f64, f64, f64) {
    let t11111 = t333 * t11110;
    let t11113 = t335 * t11110;
    let t11115 = t337 * t11110;
    let t11117 = -0.9214113627294e1_f64 * t11092 - 0.18428227254588e2_f64 * t3390 * t839 - 0.9214113627294e1_f64 * t1083 * t1310 + 0.734774460522e2_f64 * t3394 * t839 + 0.367387230261e2_f64 * t1085 * t1310 - 0.7662840944824e2_f64 * t3398 * t839 - 0.3831420472412e2_f64 * t1087 * t1310 - 0.8704e0_f64 * t11106 - 0.17408e1_f64 * t11108 - 0.8704e0_f64 * t11111 - 0.4607056813647e1_f64 * t11113 + 0.122462410087e2_f64 * t11115;
    (t11111, t11113, t11115, t11117)
}
