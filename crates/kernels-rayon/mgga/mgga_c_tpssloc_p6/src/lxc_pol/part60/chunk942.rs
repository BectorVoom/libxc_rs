//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 942/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk942(t32704: f64, t81228: f64, t81326: f64, t22704: f64, t32693: f64, t32698: f64, t6883: f64, t32705: f64, t81159: f64, t6897: f64, t8458: f64, t90544: f64) -> (f64, f64, f64, f64, f64) {
    let t120217 = t81228 * t81326 * t32704;
    let t120220 = t22704 * t81326 * t32693;
    let t120269 = t6883 * t32698;
    let t120276 = t81159 * t32705;
    let t120296 = t6897 * t90544 * t8458;
    (t120217, t120220, t120269, t120276, t120296)
}
