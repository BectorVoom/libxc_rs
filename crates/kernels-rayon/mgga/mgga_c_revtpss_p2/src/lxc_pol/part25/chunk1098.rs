//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1098/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1098(t12211: f64, t13206: f64, t1310: f64, t2371: f64, t10192: f64, t10194: f64, t10260: f64, t10263: f64, t10415: f64, t10416: f64, t10426: f64, t118: f64, t1315: f64, t1453: f64, t2320: f64, t2322: f64, t2328: f64, t2331: f64, t2372: f64, t3813: f64, t3821: f64, t4151: f64, t4254: f64, t508: f64, t511: f64, t569: f64, t649: f64, t651: f64, t671: f64) -> (f64, f64, f64) {
    let t13207 = t12211 + t13206;
    let t13216 = t1310 * t2371;
    let t13225 = t10192 * t511 - 6.0_f64 * t10194 * t508 - 2.0_f64 * t10260 * t651 - 6.0_f64 * t10263 * t651 - t10415 * t508 - 6.0_f64 * t10416 * t671 + t10426 * t569 - t118 * t13207 - 3.0_f64 * t1310 * t2320 - 6.0_f64 * t1310 * t2328 + 3.0_f64 * t1315 * t4151 - 6.0_f64 * t13216 * t651 + 3.0_f64 * t1453 * t3821 - 12.0_f64 * t2322 * t2331 - 6.0_f64 * t2322 * t2372 - 6.0_f64 * t2372 * t4254 - 3.0_f64 * t3813 * t649;
    (t13207, t13216, t13225)
}
