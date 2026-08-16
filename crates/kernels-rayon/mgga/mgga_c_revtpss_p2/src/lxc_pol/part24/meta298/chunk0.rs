//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1083/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1083(t6622: f64, t73: f64, t17934: f64, t5330: f64, t5327: f64, t5362: f64, t1803: f64, t5326: f64, t5323: f64, t12772: f64, t6639: f64, t3625: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t21040 = t6622 * t73;
    let t21049 = t17934 * t5330;
    let t21053 = t5327 * t5362;
    let t21063 = t5326 * t1803;
    let t21088 = t5323 * t5362;
    let t21090 = t12772 * t6639;
    let t21091 = t3625 * t21090;
    (t21040, t21049, t21053, t21063, t21088, t21090, t21091)
}
