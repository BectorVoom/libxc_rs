//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1332/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1332(t32698: f64, t6883: f64, t113946: f64, t1842: f64, t1992: f64, t22635: f64, t32705: f64, t81159: f64, t6897: f64, t8458: f64, t90544: f64, t114154: f64) -> (f64, f64, f64, f64, f64) {
    let t120269 = t6883 * t32698;
    let t120270 = 0.38381794893125283518e-1_f64 * t120269;
    let t120274 = 0.3289868133696452873e-1_f64 * t1992 * t22635 * t113946 * t1842;
    let t120276 = t81159 * t32705;
    let t120277 = 0.76763589786250567037e-1_f64 * t120276;
    let t120296 = t6897 * t90544 * t8458;
    let t120297 = 0.82246703342411321825e-2_f64 * t120296;
    let t120304 = 0.82246703342411321825e-2_f64 * t114154;
    (t120270, t120274, t120277, t120297, t120304)
}
