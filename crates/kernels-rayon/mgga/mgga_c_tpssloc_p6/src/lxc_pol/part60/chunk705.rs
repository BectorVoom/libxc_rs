//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 705/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk705(t6612: f64, t835: f64, t812: f64, t2627: f64, t59: f64, t240: f64, t1878: f64, t244: f64, t2230: f64, t6589: f64, t213: f64, t229: f64, t6546: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t23040 = t6612 * t835;
    let t23041 = t812 * t23040;
    let t23046 = t2627 * t59;
    let t23047 = t23046 * t240;
    let t23048 = t812 * t23047;
    let t23056 = t1878 * t244;
    let t23061 = t2230 * t6589;
    let t23062 = t23061 * t213;
    let t23069 = t6546 * t229;
    (t23041, t23046, t23048, t23056, t23062, t23069)
}
