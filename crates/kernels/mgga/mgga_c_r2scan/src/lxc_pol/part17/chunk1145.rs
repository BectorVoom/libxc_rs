//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1145/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1145<F: Float>(t12220: F, t12223: F, t31510: F, t795: F, t105: F, t3052: F, t97: F, t2526: F, t3574: F, t2850: F, t6967: F, t106: F, t8691: F) -> (F, F, F, F, F, F, F) {
    let t42372 = F::new(15.0) / F::new(8.0) * t12220;
    let t42373 = t12223 / F::new(2.0);
    let t42384 = t31510 * t795;
    let t42389 = t97 * t105 * t3052;
    let t42392 = t3574 * t2526;
    let t42403 = t6967 * t2850;
    let t42413 = t97 * t106 * t8691;
    (t42372, t42373, t42384, t42389, t42392, t42403, t42413)
}
