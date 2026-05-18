//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 791/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk791<F: Float>(t38569: F, t7192: F, t7335: F, t8355: F, t7345: F, t2185: F, t9221: F, t1997: F, t8450: F) -> (F, F, F, F, F, F) {
    let t38570 = t7192 * t38569;
    let t38608 = t7335 * t8355;
    let t38610 = t7345 * t8355;
    let t38621 = t9221 * t2185;
    let t38622 = t38621 * t1997;
    let t38623 = F::new(0.24829349937757072982e-4) * t38622;
    let t38638 = t8450 * t2185;
    (t38570, t38608, t38610, t38621, t38623, t38638)
}
