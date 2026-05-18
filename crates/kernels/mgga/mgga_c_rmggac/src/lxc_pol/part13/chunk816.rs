//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 816/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk816<F: Float>(t1971: F, t236: F, t38454: F, t5561: F, t16155: F, t8516: F, t8519: F, t615: F, t7230: F, t794: F, t9188: F, t17859: F, t7742: F) -> (F, F, F, F) {
    let t38457 = t38454 * t1971 * t236 * t5561;
    let t38460 = t8516 * t16155 * t8519;
    let t38465 = t7230 * t9188 * t236 * t615 * t794;
    let t38467 = t17859 * t7742;
    (t38457, t38460, t38465, t38467)
}
