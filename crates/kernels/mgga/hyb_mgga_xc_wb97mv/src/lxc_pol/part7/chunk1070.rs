//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1070/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1070<F: Float>(t10963: F, t10978: F, t6762: F, t6993: F, t8908: F, t9012: F, t4202: F, t6905: F, t827: F, t4230: F, t846: F, t1365: F, t3435: F, t6760: F, t8912: F, t251: F) -> (F, F, F, F, F, F, F) {
    let t11171 = -t6993 + 0.22831111111111111111e-1 * t6762 + 0.45662222222222222221e-1 * t8908 - t9012 - 0.17123333333333333333e-1 * t10963 + 0.5137e-1 * t10978;
    let t11174 = t4202 * t6905;
    let t11175 = t11174 * t827;
    let t11182 = t4230 * t846;
    let t11185 = t1365 * t3435;
    let t11192 = -t6760 + 0.23744444444444444444e-1 * t6762 + 0.47488888888888888888e-1 * t8908 - t8912 - 0.17808333333333333333e-1 * t10963 + 0.53425e-1 * t10978;
    let t11194 = 0.621814e-1 * t11192 * t251;
    (t11171, t11174, t11175, t11182, t11185, t11192, t11194)
}
