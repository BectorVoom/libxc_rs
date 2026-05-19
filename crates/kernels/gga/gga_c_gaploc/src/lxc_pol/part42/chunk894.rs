//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 894/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk894<F: Float>(t24215: F, t3553: F, t13350: F, t4342: F, t41609: F, t41612: F, t41618: F, t41623: F, t41626: F, t41629: F, t11371: F, t2482: F, t9267: F) -> (F, F, F, F, F, F, F, F, F) {
    let t46023 = F::new(2.0) * t24215 * t3553;
    let t46025 = F::new(2.0) * t4342 * t13350;
    let t46030 = F::cast_from(0.30674340763136599742e1_f64) * t41609;
    let t46031 = F::cast_from(0.14570311862489884877e2_f64) * t41612;
    let t46033 = F::cast_from(0.23833659967900284446e0_f64) * t41618;
    let t46035 = F::cast_from(0.11916829983950142223e0_f64) * t41623;
    let t46036 = F::cast_from(0.11916829983950142223e0_f64) * t41626;
    let t46037 = F::cast_from(0.11916829983950142223e0_f64) * t41629;
    let t46044 = t9267 * t11371 * t2482;
    (t46023, t46025, t46030, t46031, t46033, t46035, t46036, t46037, t46044)
}
