//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 287/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk287<F: Float>(t1163: F, t1375: F, t79: F, t963: F, t435: F, t437: F, t313: F) -> (F, F, F, F) {
    let t1376 = t1375 * t1163;
    let t1379 = t963 * t79;
    let t1382 = 0.7925e-3 * t435 * t1379 * t437;
    let t1383 = t79 * t313;
    (t1376, t1379, t1382, t1383)
}
