//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1155/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1155<F: Float>(t34321: F, t4391: F, t6964: F, t10525: F, t10526: F, t30326: F, t30330: F, t10327: F, t1580: F, t26244: F, t895: F, t1457: F, t1572: F, t31866: F, t31857: F, t31711: F) -> (F, F, F, F, F, F, F, F, F) {
    let t34324 = 0.85801175884441024006e1 * t4391 * t6964 * t34321;
    let t34327 = 0.42900587942220512002e1 * t10525 * t10526 * t34321;
    let t34328 = 0.63904876589867916128e-1 * t30326;
    let t34329 = 0.15976219147466979032e0 * t30330;
    let t34331 = 0.30674340763136599742e2 * t1580 * t10327;
    let t34333 = 0.79445533226334281487e-1 * t895 * t26244;
    let t34342 = 0.71500979903700853338e0 * t1572 * t1457 * t31866;
    let t34345 = 0.71500979903700853338e0 * t1572 * t1457 * t31857;
    let t34352 = 0.14300195980740170668e1 * t1572 * t1457 * t31711;
    (t34324, t34327, t34328, t34329, t34331, t34333, t34342, t34345, t34352)
}
