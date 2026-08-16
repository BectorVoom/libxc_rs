//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 973/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk973(t118709: f64, t118690: f64, t1510: f64, t22986: f64, t6646: f64, t1880: f64, t1894: f64, t214: f64, t28406: f64, t118727: f64, t118738: f64, t1888: f64, t232: f64, t98524: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t126442 = 0.16449340668482264365e-1_f64 * t118709;
    let t126446 = 0.6579736267392905746e-1_f64 * t22986 * t6646 * t118690 * t1510;
    let t126452 = 0.16449340668482264365e-1_f64 * t1880 * t214 * t1894 * t28406;
    let t126453 = 0.3289868133696452873e-1_f64 * t118727;
    let t126456 = 0.76763589786250567036e-1_f64 * t118738;
    let t126472 = 0.3289868133696452873e-1_f64 * t1888 * t6646 * t98524 * t232;
    (t126442, t126446, t126452, t126453, t126456, t126472)
}
