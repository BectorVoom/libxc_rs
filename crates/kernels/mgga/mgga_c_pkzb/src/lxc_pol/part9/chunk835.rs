//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 835/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk835<F: Float>(t237: F, t6297: F, t6356: F, t2461: F, t955: F, t2463: F, t418: F, t2411: F, t300: F, t2226: F, t394: F, t944: F, t2382: F, t2434: F, t2381: F, t1478: F, t154: F, t386: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t6358 = t237 * (t6297 + t6356);
    let t6359 = t2461 * t955;
    let t6362 = 1.0 / t2463 / t418;
    let t6366 = t300 * t2411;
    let t6367 = t394 * t2226;
    let t6368 = t944 * t6367;
    let t6369 = t6366 * t6368;
    let t6372 = t2434 * t2382;
    let t6373 = t2381 * t6372;
    let t6377 = t154 * t1478 * t386;
    (t6358, t6359, t6362, t6366, t6367, t6368, t6369, t6372, t6373, t6377)
}
