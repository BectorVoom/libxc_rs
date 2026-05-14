//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 292/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk292<F: Float>(t1163: F, t1398: F, t1056: F, t1349: F, t1369: F, t1374: F, t1376: F, t1382: F, t1384: F, t1388: F, t1391: F, t1397: F, t158: F, t165: F, t173: F) -> (F, F) {
    let t1399 = t1398 * t1163;
    let t1402 = t1369 + 0.11955719325063177623e-1 * t1349 * t1056 - t1374 - 0.3513e-2 * t158 * t1376 + t1382 + 0.7925e-3 * t165 * t1384 - t1388 - 0.5179538907796306876e-4 * t1391 * t1056 + t1397 + 0.50413125e-5 * t173 * t1399;
    (t1399, t1402)
}
