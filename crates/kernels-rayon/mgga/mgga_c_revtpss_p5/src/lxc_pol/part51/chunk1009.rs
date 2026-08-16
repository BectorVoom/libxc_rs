//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1009/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1009(t33942: f64, t33973: f64, t532: f64, t1450: f64, t2014: f64, t118: f64, t1502: f64, t1843: f64, t1911: f64, t1932: f64, t2007: f64, t33630: f64, t33661: f64, t33664: f64, t33666: f64, t33669: f64, t33903: f64, t33906: f64, t33910: f64, t33914: f64, t33916: f64, t33920: f64, t508: f64, t6985: f64, t7725: f64, t7746: f64, t7883: f64, t8447: f64, t8463: f64, t8557: f64, t8565: f64) -> (f64, f64, f64, f64) {
    let t33974 = t33942 + t33973;
    let t33975 = t532 * t33974;
    let t33976 = t33975 * t1450;
    let t33977 = t2014 * t33976;
    let t33982 = -t118 * t33903 - t1502 * t8557 - t1843 * t8447 + t1911 * t8565 - 2.0_f64 * t1932 * t7883 - 2.0_f64 * t2007 * t7725 - t33630 * t508 - 4.0_f64 * t6985 * t7746 + 6.0_f64 * t33661 - t33664 - t33666 + t33669 - 4.0_f64 * t33906 + 2.0_f64 * t33910 + 2.0_f64 * t33914 - t33916 + t33920 + t33977 - t8463;
    (t33974, t33975, t33976, t33982)
}
