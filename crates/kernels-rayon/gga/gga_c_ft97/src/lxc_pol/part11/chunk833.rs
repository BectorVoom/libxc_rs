//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 833/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk833(t1866: f64, t37269: f64, t446: f64, t1588: f64, t1647: f64, t7824: f64, t1882: f64, t7830: f64, t379: f64, t8183: f64, t1564: f64, t1651: f64, t1755: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t37271 = t446 * t1866 * t37269;
    let t37273 = t1647 * t1588;
    let t37275 = t446 * t7824 * t37273;
    let t37277 = t1882 * t7830;
    let t37279 = t379 * t8183;
    let t37281 = t446 * t1564 * t37279;
    let t37283 = t1651 * t1755;
    (t37271, t37273, t37275, t37277, t37279, t37281, t37283)
}
