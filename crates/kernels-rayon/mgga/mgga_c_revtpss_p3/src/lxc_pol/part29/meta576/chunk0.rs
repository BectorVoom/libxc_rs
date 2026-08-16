//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1924/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1924(t4452: f64, t92951: f64, t14719: f64, t25227: f64, t2661: f64, t14723: f64, t14774: f64, t7045: f64, t25266: f64, t4426: f64, t1561: f64, t93048: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t99023 = t92951 * t4452;
    let t99026 = t2661 * t25227 * t14719;
    let t99029 = t2661 * t25227 * t14723;
    let t99031 = t7045 * t14774;
    let t99033 = t25266 * t4426;
    let t99035 = t93048 * t1561;
    (t99023, t99026, t99029, t99031, t99033, t99035)
}
