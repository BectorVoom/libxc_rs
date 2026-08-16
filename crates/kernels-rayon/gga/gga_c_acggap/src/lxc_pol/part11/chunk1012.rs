//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1012/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1012(t33953: f64, t5150: f64, t13364: f64, t33952: f64, t154: f64, t506: f64, t7322: f64, t7326: f64, t7315: f64, t8589: f64, t2046: f64, t336: f64, t4099: f64, t579: f64) -> (f64, f64, f64, f64, f64) {
    let t33954 = t33953 * t5150;
    let t33956 = t33952 * t13364 * t33954;
    let t33960 = t7322 * t154 * t506 * t7326;
    let t33962 = t7315 * t8589;
    let t33963 = 11.0_f64 / 192.0_f64 * t33962;
    let t33966 = t2046 * t336 * t579 * t4099;
    (t33954, t33956, t33960, t33963, t33966)
}
