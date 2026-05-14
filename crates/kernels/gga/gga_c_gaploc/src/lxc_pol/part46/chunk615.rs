//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 615/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk615<F: Float>(t12695: F, t2365: F, t2033: F, t959: F, t9817: F, t10033: F, t10012: F, t935: F, t9438: F, t2684: F, t12656: F, t2685: F, t10151: F, t874: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t12696 = t2365 * t12695;
    let t12697 = t2033 * t12696;
    let t12699 = t9817 * t959;
    let t12701 = t10033 * t959;
    let t12704 = t10012 * t935;
    let t12705 = t9438 * t12704;
    let t12706 = t2684 * t12705;
    let t12708 = t2685 * t12656;
    let t12709 = t2684 * t12708;
    let t12762 = t10151 * t874;
    (t12696, t12697, t12699, t12701, t12704, t12705, t12706, t12708, t12709, t12762)
}
