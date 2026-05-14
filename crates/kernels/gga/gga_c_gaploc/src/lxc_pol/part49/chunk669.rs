//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 669/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk669<F: Float>(t12390: F, t5345: F, t5348: F, t1692: F, t3222: F, t12380: F, t713: F, t928: F, t3228: F, t871: F, t3113: F, t931: F, t12411: F, t295: F, t3276: F, t7301: F) -> (F, F, F, F, F, F, F, F) {
    let t12564 = t5345 * t12390 * t5348;
    let t12566 = t1692 * t3222;
    let t12568 = t713 * t12380;
    let t12569 = t12568 * t928;
    let t12573 = t3228 * t871;
    let t12574 = t931 * t3113;
    let t12580 = t295 * t12411;
    let t12604 = t3276 * t7301;
    (t12564, t12566, t12568, t12569, t12573, t12574, t12580, t12604)
}
