//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 987/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk987(t7901: f64, t8568: f64, t33639: f64, t508: f64, t1843: f64, t8454: f64, t13674: f64, t8599: f64, t2014: f64, t1559: f64, t31756: f64, t4364: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t33661 = t8568 * t7901;
    let t33664 = 2.0_f64 * t33639 * t508;
    let t33666 = 2.0_f64 * t8454 * t1843;
    let t33667 = t8599 * t13674;
    let t33669 = 2.0_f64 * t2014 * t33667;
    let t33674 = t4364 * t31756 * t1559;
    (t33661, t33664, t33666, t33667, t33669, t33674)
}
