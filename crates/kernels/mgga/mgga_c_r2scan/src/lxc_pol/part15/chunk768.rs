//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 768/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk768<F: Float>(t2151: F, t6395: F, t1615: F, t784: F, t2150: F, t2147: F, t113: F, t6133: F) -> (F, F, F, F) {
    let t6396 = t6395 * t2151;
    let t6398 = t784 * t1615;
    let t6399 = t6398 * t2150;
    let t6400 = t2147 * t6399;
    let t6402 = t6133 * t113;
    (t6396, t6398, t6400, t6402)
}
