//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2735/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2735(t12267: f64, t1336: f64, t1352: f64, t1380: f64, t16047: f64, t16048: f64, t16123: f64, t16433: f64, t1840: f64, t19660: f64, t19733: f64, t19743: f64, t19744: f64, t19756: f64, t3777: f64, t3793: f64, t3856: f64, t5234: f64, t5250: f64, t5334: f64, t5344: f64, t57300: f64, t57607: f64, t57704: f64, t6451: f64) -> f64 {
    let t57760 = -t1336 * t1380 * t57300 - 2.0_f64 * t1352 * t5344 * t57704 - 6.0_f64 * t16047 * t16048 * t19660 - 12.0_f64 * t16047 * t19744 * t57607 + 6.0_f64 * t19660 * t3793 * t5334 - t19660 * t3856 * t5344 + 14.0_f64 * t19743 * t3793 * t5334 - t19743 * t3856 * t5344 + 12.0_f64 * t5250 * t5334 * t57607 - 2.0_f64 * t12267 * t6451 + 2.0_f64 * t16123 * t1840 - 2.0_f64 * t16433 * t5234 - 2.0_f64 * t19733 * t3777 - 4.0_f64 * t19756 * t3777;
    t57760
}
