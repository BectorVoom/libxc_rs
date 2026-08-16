//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1009/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1009<F: Float>(t33942: F, t33973: F, t532: F, t1450: F, t2014: F, t118: F, t1502: F, t1843: F, t1911: F, t1932: F, t2007: F, t33630: F, t33661: F, t33664: F, t33666: F, t33669: F, t33903: F, t33906: F, t33910: F, t33914: F, t33916: F, t33920: F, t508: F, t6985: F, t7725: F, t7746: F, t7883: F, t8447: F, t8463: F, t8557: F, t8565: F) -> (F, F, F, F) {
    let t33974 = t33942 + t33973;
    let t33975 = t532 * t33974;
    let t33976 = t33975 * t1450;
    let t33977 = t2014 * t33976;
    let t33982 = -t118 * t33903 - t1502 * t8557 - t1843 * t8447 + t1911 * t8565 - F::cast_from(2.0_f64) * t1932 * t7883 - F::cast_from(2.0_f64) * t2007 * t7725 - t33630 * t508 - F::cast_from(4.0_f64) * t6985 * t7746 + F::cast_from(6.0_f64) * t33661 - t33664 - t33666 + t33669 - F::cast_from(4.0_f64) * t33906 + F::cast_from(2.0_f64) * t33910 + F::cast_from(2.0_f64) * t33914 - t33916 + t33920 + t33977 - t8463;
    (t33974, t33975, t33976, t33982)
}
