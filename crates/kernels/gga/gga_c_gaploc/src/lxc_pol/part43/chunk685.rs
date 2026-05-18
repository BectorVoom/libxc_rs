//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 685/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk685<F: Float>(t10012: F, t935: F, t9438: F, t2684: F, t12656: F, t2685: F, t2787: F, t3116: F) -> (F, F, F, F, F, F) {
    let t12704 = t10012 * t935;
    let t12705 = t9438 * t12704;
    let t12706 = t2684 * t12705;
    let t12708 = t2685 * t12656;
    let t12709 = t2684 * t12708;
    let t12766 = t2787 * t3116;
    (t12704, t12705, t12706, t12708, t12709, t12766)
}
