//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2014/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2014(t21082: f64, t482: f64, t371: f64, t372: f64, t5323: f64, t5362: f64, t12772: f64, t6639: f64, t3625: f64, t1263: f64, t6573: f64, t1122: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t21083 = t482 * t21082;
    let t21085 = t371 * t372 * t21083;
    let t21088 = t5323 * t5362;
    let t21090 = t12772 * t6639;
    let t21091 = t3625 * t21090;
    let t21093 = t1263 * t6573;
    let t21094 = t21093 * t1122;
    (t21083, t21085, t21088, t21090, t21091, t21093, t21094)
}
