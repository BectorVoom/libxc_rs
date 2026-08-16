//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1937/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1937(t18521: f64, t27261: f64, t25222: f64, t6030: f64, t18423: f64, t25234: f64, t5993: f64, t18414: f64, t2661: f64, t93082: f64, t18418: f64, t25227: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t106012 = t27261 * t18521;
    let t106014 = t25222 * t6030;
    let t106022 = t25234 * t18423;
    let t106024 = t25222 * t5993;
    let t106030 = t2661 * t93082 * t18414;
    let t106033 = t2661 * t25227 * t18418;
    (t106012, t106014, t106022, t106024, t106030, t106033)
}
