//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1198/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1198(t21700: f64, t21720: f64, t21735: f64, t21769: f64, t1137: f64, t6167: f64, t13787: f64, t1788: f64, t3409: f64, t5991: f64, t1083: f64, t1131: f64, t1298: f64, t13230: f64, t13235: f64, t13253: f64, t15814: f64, t16745: f64, t1772: f64, t21663: f64, t335: f64, t336: f64, t367: f64, t368: f64, t372: f64, t398: f64, t418: f64, t4256: f64, t4630: f64, t5641: f64) -> (f64, f64) {
    let t21771 = t21700 + t21720 + t21735 + t21769;
    let t21776 = t1137 * t6167;
    let t21778 = t13787 * t1788;
    let t21790 = t3409 * t5991;
    let t21795 = 0.68598428988911579156e-2_f64 * t21663 + t15814 * t4256 * t5641 * t372 / 2.0_f64 - t367 * t336 * t368 * t21771 / 96.0_f64 + 7.0_f64 / 72.0_f64 * t21776 + 35.0_f64 / 72.0_f64 * t21778 - t335 * t336 * t4630 * t1298 / 12.0_f64 + 0.20007875121765877254e-2_f64 * t16745 - 0.85748036236139473944e-3_f64 * t418 * t398 * t1083 * t1772 * t1131 - 0.16006300097412701803e-1_f64 * t21790 - 0.17149607247227894789e-2_f64 * t13230 - 0.85748036236139473944e-3_f64 * t13235 + 0.17149607247227894789e-2_f64 * t13253;
    (t21771, t21795)
}
