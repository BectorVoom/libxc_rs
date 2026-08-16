//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1180/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1180(t1071: f64, t1745: f64, t1154: f64, t2630: f64, t13480: f64, t5134: f64, t119: f64, t41: f64, t85: f64, t13511: f64, t5142: f64, t1018: f64, t1083: f64) -> (f64, f64, f64, f64, f64) {
    let t14999 = t1745 * t1071;
    let t15001 = t1154 * t14999 * t2630;
    let t15004 = t5134 * t13480;
    let t15007 = t119 * t41;
    let t15008 = t85 * t15007;
    let t15009 = t5142 * t13511;
    let t15012 = t1018 * t1083;
    (t15001, t15004, t15008, t15009, t15012)
}
