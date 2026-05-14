//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1292/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1292<F: Float>(t6118: F, t8753: F, t2667: F, t7391: F, t2177: F, t9302: F, t25766: F, t2691: F, t7624: F, t26319: F, t30158: F, t2133: F, t22709: F, t8736: F, t2155: F, t29726: F, t8088: F) -> (F, F, F, F, F, F, F) {
    let t30559 = t6118 * t8753;
    let t30570 = t2667 * t7391;
    let t30572 = t2177 * t9302;
    let t30577 = t25766 * t2691 * t7624;
    let t30579 = t26319 * t30158;
    let t30599 = t2133 * t22709 * t8736;
    let t30626 = t2155 * t8088 * t29726;
    (t30559, t30570, t30572, t30577, t30579, t30599, t30626)
}
