//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 931/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk931<F: Float>(t35195: F, t40450: F, t1184: F, t1971: F, t236: F, t36489: F, t40064: F, t2868: F, t7779: F, t2186: F, t8597: F, t2412: F, t7404: F) -> (F, F, F, F, F) {
    let t40451 = t40450 * t35195;
    let t40456 = t36489 * t1971 * t236 * t40064 * t1184;
    let t40458 = t2868 * t7779;
    let t40479 = t2186 * t8597;
    let t40481 = t2412 * t7404;
    (t40451, t40456, t40458, t40479, t40481)
}
