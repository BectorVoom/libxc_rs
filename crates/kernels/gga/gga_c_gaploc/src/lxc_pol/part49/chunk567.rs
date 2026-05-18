//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 567/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk567<F: Float>(t286: F, t708: F, t9095: F, t1687: F, t9099: F, t5337: F, t5340: F, t9106: F, t5345: F, t5348: F, t2519: F, t3220: F) -> (F, F, F, F, F) {
    let t9664 = t9095 * t286 * t708;
    let t9666 = t9099 * t1687;
    let t9669 = t9106 * t5337 * t5340;
    let t9672 = t5345 * t9106 * t5348;
    let t9674 = t3220 * t2519;
    (t9664, t9666, t9669, t9672, t9674)
}
