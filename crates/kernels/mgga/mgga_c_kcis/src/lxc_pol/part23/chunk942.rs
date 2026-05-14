//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 942/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk942<F: Float>(t27387: F, t4165: F, t1394: F, t3955: F, t7931: F, t303: F, t1386: F, t3999: F, t4001: F, t6176: F, t3723: F, t553: F, t4007: F, t7914: F, t12286: F, t1598: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t27388 = t27387 * t4165;
    let t27389 = t1394 * t27388;
    let t27391 = t7931 * t3955;
    let t27392 = t303 * t27391;
    let t27394 = t3999 * t1386;
    let t27395 = t27394 * t4001;
    let t27396 = t6176 * t27395;
    let t27399 = t553 * t3723;
    let t27400 = t303 * t27399;
    let t27402 = t7914 * t4007;
    let t27403 = t6176 * t27402;
    let t27410 = t12286 * t1598;
    (t27388, t27389, t27391, t27392, t27395, t27396, t27399, t27400, t27402, t27403, t27410)
}
