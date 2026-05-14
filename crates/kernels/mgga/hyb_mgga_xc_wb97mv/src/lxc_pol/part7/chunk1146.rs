//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1146/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1146<F: Float>(t1029: F, t23613: F, t14: F, t22469: F, t237: F, t23606: F, t23608: F, t23611: F, t23614: F, t23617: F, t23622: F, t23624: F, t23626: F, t1078: F, t1085: F, t1099: F) -> (F, F, F, F) {
    let t23628 = t1029 * t23613;
    let t23631 = t237 * t14 * t22469;
    let t23633 = -0.28769444444444444444e1 * t23606 + 0.27618666666666666667e2 * t23608 - 0.10229135802469135803e2 * t23611 + 0.89504938271604938273e1 * t23614 + 0.31310740740740740741e1 * t23617 + 0.366775e-1 * t23622 - 0.58684e0 * t23624 + 0.65204444444444444445e0 * t23626 + 0.5705388888888888889e0 * t23628 + 0.13490888888888888889e1 * t23631;
    let t23637 = 0.5848223622634646207e0 * t1099 * t1078 * t23633 * t1085;
    (t23628, t23631, t23633, t23637)
}
