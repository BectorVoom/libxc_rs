//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1378/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1378(t23185: f64, t33457: f64, t82074: f64, t1888: f64, t23270: f64, t31332: f64, t4300: f64, t2048: f64, t254: f64, t225: f64, t33414: f64, t1880: f64, t23237: f64, t33408: f64) -> (f64, f64, f64, f64, f64) {
    let t121444 = t23185 * t82074 * t33457;
    let t121448 = t1888 * t23270 * t31332 * t4300;
    let t121451 = t2048 * t254;
    let t121454 = t33414 * t225;
    let t121457 = t1880 * t23237 * t33408;
    (t121444, t121448, t121451, t121454, t121457)
}
