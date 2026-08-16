//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1725/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1725(t23041: f64, t831: f64, t2627: f64, t59: f64, t240: f64, t812: f64, t2617: f64, t6613: f64, t1878: f64, t244: f64, t2230: f64, t6589: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t23042 = t23041 * t831;
    let t23046 = t2627 * t59;
    let t23047 = t23046 * t240;
    let t23048 = t812 * t23047;
    let t23053 = t2617 * t6613;
    let t23056 = t1878 * t244;
    let t23061 = t2230 * t6589;
    (t23042, t23046, t23047, t23048, t23053, t23056, t23061)
}
