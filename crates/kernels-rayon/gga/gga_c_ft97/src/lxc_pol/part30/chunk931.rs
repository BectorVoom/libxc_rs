//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 931/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk931(t33253: f64, t683: f64, t1403: f64, t33244: f64, t681: f64, t2371: f64, t33452: f64, t24178: f64, t7437: f64, t2567: f64, t7536: f64, t33792: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t140594 = t683 * t33253;
    let t140605 = t1403 * t681 * t33244;
    let t140627 = t2371 * t33452;
    let t140649 = t7437 * t24178;
    let t140653 = t7536 * t2567;
    let t140664 = t1403 * t681 * t33792;
    (t140594, t140605, t140627, t140649, t140653, t140664)
}
