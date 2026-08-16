//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1066/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1066(t33942: f64, t33973: f64, t532: f64, t1450: f64, t2014: f64, t2042: f64, t7944: f64, t2040: f64, t7950: f64, t7953: f64, t1916: f64, t8611: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t33974 = t33942 + t33973;
    let t33975 = t532 * t33974;
    let t33976 = t33975 * t1450;
    let t33977 = t2014 * t33976;
    let t33996 = t7944 * t2042;
    let t33998 = t2040 * t7950;
    let t34000 = t2040 * t7953;
    let t34003 = 6.0_f64 * t1916 * t8611;
    (t33974, t33975, t33976, t33977, t33996, t33998, t34000, t34003)
}
