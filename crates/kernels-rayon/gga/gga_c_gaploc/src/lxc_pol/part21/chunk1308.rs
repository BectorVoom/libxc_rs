//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1308/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1308(t34321: f64, t4391: f64, t6964: f64, t10525: f64, t10526: f64, t30326: f64, t30330: f64, t10327: f64, t1580: f64, t26244: f64, t895: f64, t1457: f64, t1572: f64, t31866: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t34324 = 0.85801175884441024006e1_f64 * t4391 * t6964 * t34321;
    let t34327 = 0.42900587942220512002e1_f64 * t10525 * t10526 * t34321;
    let t34328 = 0.63904876589867916128e-1_f64 * t30326;
    let t34329 = 0.15976219147466979032e0_f64 * t30330;
    let t34331 = 0.30674340763136599742e2_f64 * t1580 * t10327;
    let t34333 = 0.79445533226334281487e-1_f64 * t895 * t26244;
    let t34342 = 0.71500979903700853338e0_f64 * t1572 * t1457 * t31866;
    (t34324, t34327, t34328, t34329, t34331, t34333, t34342)
}
