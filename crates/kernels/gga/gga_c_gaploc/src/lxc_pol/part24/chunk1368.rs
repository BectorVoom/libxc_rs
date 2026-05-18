//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1368/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1368<F: Float>(t34321: F, t4391: F, t6964: F, t10525: F, t10526: F, t30326: F, t30330: F, t10327: F, t1580: F, t26244: F, t895: F, t1457: F, t1572: F, t31866: F) -> (F, F, F, F, F, F, F) {
    let t34324 = F::new(0.85801175884441024006e1) * t4391 * t6964 * t34321;
    let t34327 = F::new(0.42900587942220512002e1) * t10525 * t10526 * t34321;
    let t34328 = F::new(0.63904876589867916128e-1) * t30326;
    let t34329 = F::new(0.15976219147466979032e0) * t30330;
    let t34331 = F::new(0.30674340763136599742e2) * t1580 * t10327;
    let t34333 = F::new(0.79445533226334281487e-1) * t895 * t26244;
    let t34342 = F::new(0.71500979903700853338e0) * t1572 * t1457 * t31866;
    (t34324, t34327, t34328, t34329, t34331, t34333, t34342)
}
