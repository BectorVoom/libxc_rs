//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1178/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1178(t30066: f64, t30109: f64, t532: f64, t1450: f64, t2014: f64, t1518: f64, t7883: f64, t2007: f64, t5920: f64, t1868: f64, t1907: f64, t8717: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t30110 = t30066 + t30109;
    let t30111 = t532 * t30110;
    let t30112 = t30111 * t1450;
    let t30113 = t2014 * t30112;
    let t30116 = t7883 * t1518;
    let t30119 = t2007 * t5920;
    let t30122 = t1868 * t1907;
    let t30123 = t8717 * t30122;
    (t30110, t30111, t30112, t30113, t30116, t30119, t30122, t30123)
}
