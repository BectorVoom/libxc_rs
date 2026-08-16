//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 950/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk950<F: Float>(t25548: F, t6800: F, t23635: F, t1629: F, t6743: F, t884: F, t4684: F, t7619: F, t1610: F, t1920: F, t1953: F, t23633: F, t23666: F, t25530: F, t25536: F, t25541: F, t25545: F, t3200: F, t4615: F, t4669: F, t6797: F, t6811: F, t6813: F) -> (F, F, F) {
    let t25549 = t25548 * t6800;
    let t25550 = t23635 * t25549;
    let t25553 = t6743 * t1629;
    let t25554 = t6800 * t884;
    let t25555 = t25553 * t25554;
    let t25558 = t7619 * t4684;
    let t25560 = F::cast_from(0.27415567780803773942e-2_f64) * t25530 + t4615 * t1953 + t1610 * t6813 + t4669 * t6811 + F::cast_from(0.82246703342411321825e-2_f64) * t1920 * t25536 + F::cast_from(0.27415567780803773942e-2_f64) * t23666 + F::cast_from(0.82246703342411321825e-2_f64) * t6797 * t25541 + F::cast_from(0.82246703342411321825e-2_f64) * t6797 * t25545 + F::cast_from(0.27415567780803773942e-2_f64) * t23633 * t25550 + F::cast_from(0.27415567780803773942e-2_f64) * t23633 * t25555 - t3200 * t25558;
    (t25549, t25554, t25560)
}
