//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 749/952 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk749<F: Float>(t35455: F, t8451: F, t7421: F, t8571: F, t2185: F, t9221: F, t1997: F, t7905: F, t1987: F, t38351: F, t38355: F, t7682: F, t1990: F, t7687: F, t8450: F, t2004: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t38617 = t8451 * t35455;
    let t38619 = t8571 * t7421;
    let t38621 = t9221 * t2185;
    let t38622 = t38621 * t1997;
    let t38623 = 0.24829349937757072982e-4 * t38622;
    let t38624 = t8571 * t7905;
    let t38626 = t38351 * t1987;
    let t38628 = t38355 * t1987;
    let t38630 = t8571 * t7682;
    let t38632 = t38351 * t1990;
    let t38634 = t38355 * t1990;
    let t38636 = t8571 * t7687;
    let t38638 = t8450 * t2185;
    let t38639 = t38638 * t2004;
    (t38617, t38619, t38623, t38624, t38626, t38628, t38630, t38632, t38634, t38636, t38638, t38639)
}
