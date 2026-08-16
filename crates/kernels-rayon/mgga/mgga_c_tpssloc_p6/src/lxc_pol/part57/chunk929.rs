//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 929/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk929(t1437: f64, t31: f64, t7440: f64, t79: f64, t22751: f64, t32731: f64, t1377: f64, t7749: f64, t32704: f64, t81228: f64, t81326: f64, t22704: f64, t32693: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t119878 = t1437 * t31;
    let t119942 = t79 * t7440;
    let t120179 = t22751 * t32731;
    let t120197 = t1377 * t7749;
    let t120217 = t81228 * t81326 * t32704;
    let t120220 = t22704 * t81326 * t32693;
    (t119878, t119942, t120179, t120197, t120217, t120220)
}
