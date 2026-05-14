//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1013/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1013<F: Float>(t1115: F, t2526: F, t3270: F, t12197: F, t1561: F, t12366: F, t12367: F, t12368: F, t12220: F, t12223: F, t31510: F, t795: F, t105: F, t3052: F, t97: F, t3574: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t42318 = t3270 * t1115 * t2526;
    let t42331 = t1561 * t12197;
    let t42369 = 2.0 * t12366;
    let t42370 = 2.0 * t12367;
    let t42371 = 2.0 * t12368;
    let t42372 = 15.0 / 8.0 * t12220;
    let t42373 = t12223 / 2.0;
    let t42384 = t31510 * t795;
    let t42389 = t97 * t105 * t3052;
    let t42392 = t3574 * t2526;
    (t42318, t42331, t42369, t42370, t42371, t42372, t42373, t42384, t42389, t42392)
}
