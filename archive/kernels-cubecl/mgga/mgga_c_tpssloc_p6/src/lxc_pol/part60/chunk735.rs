//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 735/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk735<F: Float>(t23164: F, t25345: F, t225: F, t7511: F, t2752: F, t7540: F, t10143: F, t25: F, t28: F, t1437: F, t1864: F, t1410: F, t2240: F) -> (F, F, F, F, F, F, F) {
    let t25346 = t23164 * t25345;
    let t25348 = t7511 * t225;
    let t25358 = t7540 * t2752;
    let t25373 = t10143 * t25;
    let t25927 = t10143 * t28;
    let t26012 = t1864 * t1437;
    let t26016 = t2240 * t1410;
    (t25346, t25348, t25358, t25373, t25927, t26012, t26016)
}
