//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 688/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk688<F: Float>(t205: F, t5194: F, t242: F, t3788: F, t1336: F, t557: F, t67: F, t246: F, t546: F, t68: F, t3787: F, t544: F) -> (F, F, F, F, F, F) {
    let t5195 = t205 * t5194;
    let t5245 = t3788 * t242;
    let t5246 = t1336 * t5245;
    let t5247 = t557 * t67;
    let t5248 = t5247 * t246;
    let t5278 = t546 * t68;
    let t5333 = t68 * t3787;
    let t5334 = t544 * t5333;
    (t5195, t5246, t5247, t5248, t5278, t5334)
}
