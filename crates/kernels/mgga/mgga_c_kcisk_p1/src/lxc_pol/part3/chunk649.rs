//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 649/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk649<F: Float>(t10399: F, t6666: F, t5192: F, t5182: F, t6675: F, t6674: F, t140: F, t3737: F, t5180: F, t5196: F, t1797: F, t1862: F) -> (F, F, F, F, F) {
    let t10400 = t6666 * t10399;
    let t10401 = t5192 * t10400;
    let t10402 = t5182 * t10401;
    let t10404 = t6675 * t10399;
    let t10405 = t5192 * t10404;
    let t10406 = t6674 * t10405;
    let t10409 = t140 * t3737 * t5180;
    let t10410 = t10409 * t5196;
    let t10412 = t1797 * t1862;
    (t10402, t10406, t10409, t10410, t10412)
}
