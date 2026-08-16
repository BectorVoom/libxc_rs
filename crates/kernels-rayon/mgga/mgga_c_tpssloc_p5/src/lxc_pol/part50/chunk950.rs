//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 950/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk950(t25548: f64, t6800: f64, t23635: f64, t1629: f64, t6743: f64, t884: f64, t4684: f64, t7619: f64, t1610: f64, t1920: f64, t1953: f64, t23633: f64, t23666: f64, t25530: f64, t25536: f64, t25541: f64, t25545: f64, t3200: f64, t4615: f64, t4669: f64, t6797: f64, t6811: f64, t6813: f64) -> (f64, f64, f64) {
    let t25549 = t25548 * t6800;
    let t25550 = t23635 * t25549;
    let t25553 = t6743 * t1629;
    let t25554 = t6800 * t884;
    let t25555 = t25553 * t25554;
    let t25558 = t7619 * t4684;
    let t25560 = 0.27415567780803773942e-2_f64 * t25530 + t4615 * t1953 + t1610 * t6813 + t4669 * t6811 + 0.82246703342411321825e-2_f64 * t1920 * t25536 + 0.27415567780803773942e-2_f64 * t23666 + 0.82246703342411321825e-2_f64 * t6797 * t25541 + 0.82246703342411321825e-2_f64 * t6797 * t25545 + 0.27415567780803773942e-2_f64 * t23633 * t25550 + 0.27415567780803773942e-2_f64 * t23633 * t25555 - t3200 * t25558;
    (t25549, t25554, t25560)
}
