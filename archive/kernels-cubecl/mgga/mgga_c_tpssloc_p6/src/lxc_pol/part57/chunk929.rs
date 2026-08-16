//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 929/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk929<F: Float>(t1437: F, t31: F, t7440: F, t79: F, t22751: F, t32731: F, t1377: F, t7749: F, t32704: F, t81228: F, t81326: F, t22704: F, t32693: F) -> (F, F, F, F, F, F) {
    let t119878 = t1437 * t31;
    let t119942 = t79 * t7440;
    let t120179 = t22751 * t32731;
    let t120197 = t1377 * t7749;
    let t120217 = t81228 * t81326 * t32704;
    let t120220 = t22704 * t81326 * t32693;
    (t119878, t119942, t120179, t120197, t120217, t120220)
}
