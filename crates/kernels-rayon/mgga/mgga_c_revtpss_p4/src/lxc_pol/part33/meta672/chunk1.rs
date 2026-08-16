//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2202/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2202(t25082: f64, t30122: f64, t32113: f64, t1448: f64, t6781: f64, t28196: f64, t98495: f64, t1353: f64, t28197: f64, t28167: f64, t8717: f64, t2014: f64, t25190: f64, t29494: f64) -> (f64, f64, f64, f64, f64) {
    let t109095 = 6.0_f64 * t25082 * t32113 * t30122;
    let t109096 = t6781 * t1448;
    let t109099 = 6.0_f64 * t28196 * t98495 * t109096;
    let t109100 = t6781 * t1353;
    let t109103 = 6.0_f64 * t25082 * t28197 * t109100;
    let t109104 = t30122 * t1353;
    let t109107 = 12.0_f64 * t28167 * t8717 * t109104;
    let t109110 = 3.0_f64 * t2014 * t25190 * t29494;
    (t109095, t109099, t109103, t109107, t109110)
}
