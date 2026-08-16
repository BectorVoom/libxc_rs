//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2719/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2719(t225: f64, t70208: f64, t480: f64, t17289: f64, t1803: f64, t1222: f64, t6652: f64, t697: f64, t12916: f64, t17709: f64, t20958: f64, t1235: f64, t371: f64, t6645: f64, t676: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t70209 = t70208 * t225;
    let t70210 = t70209 * t480;
    let t70221 = t17289 * t1803;
    let t70225 = t1222 * t697 * t6652;
    let t70250 = t17709 * t12916 * t20958;
    let t70263 = t1235 * t371 * t676 * t6645;
    (t70209, t70210, t70221, t70225, t70250, t70263)
}
