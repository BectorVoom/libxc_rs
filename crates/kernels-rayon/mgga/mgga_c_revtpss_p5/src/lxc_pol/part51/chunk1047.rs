//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1047/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1047(t3167: f64, t31997: f64, t31998: f64, t31919: f64, t32013: f64, t1078: f64, t373: f64, t31949: f64, t32009: f64, t1032: f64, t1976: f64, t994: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t120318 = t31997 * t31998 * t3167;
    let t120321 = t31919 * t32013;
    let t120322 = t373 * t1078;
    let t120329 = t32009 * t31949;
    let t120334 = t1976 * t1032;
    let t120335 = t994 * t120334;
    (t120318, t120321, t120322, t120329, t120334, t120335)
}
