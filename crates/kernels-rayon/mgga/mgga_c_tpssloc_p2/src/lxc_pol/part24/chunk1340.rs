//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1340/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1340(t23186: f64, t82031: f64, t1902: f64, t2678: f64, t22723: f64, t23163: f64, t23165: f64, t10046: f64, t1880: f64, t1894: f64, t214: f64, t1879: f64, t80845: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t82032 = t82031 * t23186;
    let t82034 = t1902 * t2678;
    let t82038 = t22723 * t23163;
    let t82039 = t82038 * t23165;
    let t82043 = t1880 * t214 * t1894 * t10046;
    let t82045 = t80845 * t1879;
    (t82032, t82034, t82038, t82039, t82043, t82045)
}
