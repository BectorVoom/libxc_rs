//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1198/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1198(t112892: f64, t32792: f64, t6547: f64, t1880: f64, t25329: f64, t6553: f64, t6571: f64, t112660: f64, t7488: f64, t112899: f64, t22986: f64, t25054: f64) -> (f64, f64, f64, f64, f64) {
    let t118851 = 0.82246703342411321825e-2_f64 * t112892;
    let t118858 = t6547 * t32792;
    let t118859 = 0.38381794893125283518e-1_f64 * t118858;
    let t118871 = 0.16449340668482264365e-1_f64 * t1880 * t6553 * t6571 * t25329;
    let t118874 = 0.16449340668482264365e-1_f64 * t1880 * t112660 * t7488;
    let t118877 = 0.3289868133696452873e-1_f64 * t22986 * t112899 * t25054;
    (t118851, t118859, t118871, t118874, t118877)
}
