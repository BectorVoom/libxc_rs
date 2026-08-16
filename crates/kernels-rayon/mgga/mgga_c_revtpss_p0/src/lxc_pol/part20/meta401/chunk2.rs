//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1490/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1490(t11752: f64, t3241: f64, t11755: f64, t1011: f64, t3247: f64, t697: f64, t3254: f64, t11789: f64, t11937: f64, t225: f64, t42051: f64, t366: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t42249 = t3241 * t11752;
    let t42251 = t3241 * t11755;
    let t42254 = t1011 * t697 * t3247;
    let t42257 = t1011 * t697 * t3254;
    let t42259 = t11789 * t11937;
    let t42261 = t42051 * t225;
    let t42262 = t42261 * t366;
    (t42249, t42251, t42254, t42257, t42259, t42261, t42262)
}
