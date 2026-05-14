//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1093/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1093<F: Float>(t1353: F, t3696: F, t39284: F, t39289: F, t39295: F, t39298: F, t39303: F, t39306: F, t39314: F, t39317: F, t39321: F, t39326: F, t39330: F, t39334: F, t39338: F, t39342: F, t39344: F) -> (F,) {
    let t40726 = t1353 * t3696 + t39284 + t39289 + t39295 + t39298 + t39303 + t39306 - t39314 - t39317 - t39321 + t39326 - t39330 - t39334 + t39338 + t39342 - t39344;
    (t40726,)
}
