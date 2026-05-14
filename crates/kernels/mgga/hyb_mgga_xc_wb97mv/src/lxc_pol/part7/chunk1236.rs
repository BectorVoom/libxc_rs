//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1236/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1236<F: Float>(t136: F, t2003: F, t4068: F, t4110: F, t2022: F, t4115: F, t10888: F, t676: F, t10877: F, t549: F, t10862: F, t679: F, t2029: F, t10894: F, t10845: F, t1234: F, t8630: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t30153 = t136 * t2003 * t4068;
    let t30156 = t136 * t2003 * t4110;
    let t30158 = t4115 * t2022;
    let t30167 = t676 * t10888;
    let t30170 = t136 * t549 * t10877;
    let t30172 = t10862 * t679;
    let t30174 = t4115 * t2029;
    let t30176 = t676 * t10894;
    let t30179 = t136 * t549 * t10845;
    let t30181 = t1234 * t8630;
    (t30153, t30156, t30158, t30167, t30170, t30172, t30174, t30176, t30179, t30181)
}
