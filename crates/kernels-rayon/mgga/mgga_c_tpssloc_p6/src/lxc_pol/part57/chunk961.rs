//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 961/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk961(t118678: f64, t1888: f64, t232: f64, t6646: f64, t98541: f64, t22996: f64, t2632: f64, t118709: f64, t118690: f64, t1510: f64, t22986: f64, t1880: f64, t1894: f64, t214: f64, t28406: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t126433 = 0.76763589786250567036e-1_f64 * t118678;
    let t126437 = 0.16449340668482264365e-1_f64 * t1888 * t6646 * t98541 * t232;
    let t126441 = 0.3289868133696452873e-1_f64 * t1888 * t22996 * t98541 * t2632;
    let t126442 = 0.16449340668482264365e-1_f64 * t118709;
    let t126446 = 0.6579736267392905746e-1_f64 * t22986 * t6646 * t118690 * t1510;
    let t126452 = 0.16449340668482264365e-1_f64 * t1880 * t214 * t1894 * t28406;
    (t126433, t126437, t126441, t126442, t126446, t126452)
}
