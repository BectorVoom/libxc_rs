//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1190/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1190<F: Float>(t112892: F, t32792: F, t6547: F, t1880: F, t25329: F, t6553: F, t6571: F, t112660: F, t7488: F, t112899: F, t22986: F, t25054: F) -> (F, F, F, F, F) {
    let t118851 = F::cast_from(0.82246703342411321825e-2_f64) * t112892;
    let t118858 = t6547 * t32792;
    let t118859 = F::cast_from(0.38381794893125283518e-1_f64) * t118858;
    let t118871 = F::cast_from(0.16449340668482264365e-1_f64) * t1880 * t6553 * t6571 * t25329;
    let t118874 = F::cast_from(0.16449340668482264365e-1_f64) * t1880 * t112660 * t7488;
    let t118877 = F::cast_from(0.3289868133696452873e-1_f64) * t22986 * t112899 * t25054;
    (t118851, t118859, t118871, t118874, t118877)
}
