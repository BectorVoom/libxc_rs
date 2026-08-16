//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 958/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk958(t1880: f64, t8335: f64, t98133: f64, t118830: f64, t1888: f64, t23270: f64, t30633: f64, t5657: f64, t118910: f64, t7488: f64, t25038: f64, t30622: f64, t5527: f64) -> (f64, f64, f64, f64, f64) {
    let t126363 = 0.16449340668482264365e-1_f64 * t1880 * t98133 * t8335;
    let t126368 = 0.16449340668482264365e-1_f64 * t118830;
    let t126372 = 0.3289868133696452873e-1_f64 * t1888 * t23270 * t30633 * t5657;
    let t126385 = 0.3289868133696452873e-1_f64 * t1880 * t118910 * t7488;
    let t126398 = 0.9869604401089358619e-1_f64 * t25038 * t23270 * t30622 * t5527;
    (t126363, t126368, t126372, t126385, t126398)
}
