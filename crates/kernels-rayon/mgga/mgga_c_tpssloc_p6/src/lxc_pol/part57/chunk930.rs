//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 930/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk930(t32698: f64, t6883: f64, t32705: f64, t81159: f64, t6897: f64, t8458: f64, t90544: f64, t114172: f64, t22892: f64, t7691: f64, t3886: f64, t7749: f64) -> (f64, f64, f64, f64, f64) {
    let t120269 = t6883 * t32698;
    let t120276 = t81159 * t32705;
    let t120296 = t6897 * t90544 * t8458;
    let t120308 = t22892 * t114172 * t7691;
    let t120317 = t3886 * t7749;
    (t120269, t120276, t120296, t120308, t120317)
}
