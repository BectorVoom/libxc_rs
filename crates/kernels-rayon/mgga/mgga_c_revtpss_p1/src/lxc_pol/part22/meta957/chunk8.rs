//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3215/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3215(t5: f64, t60692: f64, t61007: f64, t117: f64, t10416: f64, t1310: f64, t13425: f64, t13429: f64, t13435: f64, t1502: f64, t1518: f64, t18153: f64, t18220: f64, t18242: f64, t1843: f64, t21658: f64, t21814: f64, t2320: f64, t2322: f64, t3813: f64, t4246: f64, t508: f64, t5517: f64, t5877: f64, t5921: f64, t60650: f64, t60656: f64, t649: f64, t651: f64, t6765: f64) -> (f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t61009 = piecewise3(t8, 0.0_f64, t60692 + t61007);
    let t61010 = t61009 * t117;
    let t61014 = -4.0_f64 * t1518 * t18153 * t651 - 2.0_f64 * t10416 * t5921 - 4.0_f64 * t1310 * t18220 - 2.0_f64 * t1310 * t21814 - 2.0_f64 * t13425 * t1843 - 4.0_f64 * t13429 * t1843 - 4.0_f64 * t13435 * t5921 - 2.0_f64 * t1502 * t18153 - 4.0_f64 * t18242 * t2322 - 2.0_f64 * t21658 * t649 - t2320 * t6765 - t3813 * t5877 - 4.0_f64 * t4246 * t5517 - 2.0_f64 * t508 * t60650 - 2.0_f64 * t508 * t60656 - t508 * t61010;
    (t61010, t61014)
}
