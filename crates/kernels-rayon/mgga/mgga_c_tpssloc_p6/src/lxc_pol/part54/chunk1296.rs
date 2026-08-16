//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1296/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1296(t31550: f64, t81228: f64, t81326: f64, t31551: f64, t81159: f64, t115352: f64, t6897: f64, t6907: f64, t3886: f64, t7213: f64, t225: f64, t31585: f64) -> (f64, f64, f64, f64, f64) {
    let t115586 = t81228 * t81326 * t31550;
    let t115596 = t81159 * t31551;
    let t115601 = t6897 * t115352 * t6907;
    let t115614 = t3886 * t7213;
    let t115619 = t31585 * t225;
    (t115586, t115596, t115601, t115614, t115619)
}
