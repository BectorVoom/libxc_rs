//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3242/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3242(t117: f64, t85307: f64, t118: f64, t13426: f64, t18227: f64, t18232: f64, t18235: f64, t18242: f64, t18245: f64, t1843: f64, t21814: f64, t25043: f64, t4248: f64, t4297: f64, t508: f64, t5921: f64, t649: f64, t651: f64, t670: f64, t671: f64, t75931: f64, t75941: f64, t81110: f64, t85032: f64) -> (f64, f64) {
    let t85308 = t85307 * t117;
    let t85312 = -6.0_f64 * t18245 * t4297 - 2.0_f64 * t651 * t25043 * t670 - 2.0_f64 * t651 * t508 * t75931 - 6.0_f64 * t13426 * t5921 - 6.0_f64 * t18227 * t5921 - 6.0_f64 * t4248 * t18242 - 2.0_f64 * t75941 * t671 - 12.0_f64 * t4248 * t18235 - 6.0_f64 * t4248 * t18232 - t649 * t25043 - t118 * (t81110 + t85032) - t85308 * t508 - 3.0_f64 * t21814 * t1843;
    (t85308, t85312)
}
