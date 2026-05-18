//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 945/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk945<F: Float>(t13350: F, t4342: F, t41609: F, t41612: F, t41618: F, t41623: F, t41626: F, t41629: F, t11371: F, t2482: F, t9267: F, t2478: F, t3536: F, t6576: F) -> (F, F, F, F, F, F, F, F, F) {
    let t46025 = F::new(2.0) * t4342 * t13350;
    let t46030 = F::new(0.30674340763136599742e1) * t41609;
    let t46031 = F::new(0.14570311862489884877e2) * t41612;
    let t46033 = F::new(0.23833659967900284446e0) * t41618;
    let t46035 = F::new(0.11916829983950142223e0) * t41623;
    let t46036 = F::new(0.11916829983950142223e0) * t41626;
    let t46037 = F::new(0.11916829983950142223e0) * t41629;
    let t46044 = t9267 * t11371 * t2482;
    let t46045 = F::new(0.9585731488480187419e0) * t46044;
    let t46047 = t6576 * t3536 * t2478;
    (t46025, t46030, t46031, t46033, t46035, t46036, t46037, t46045, t46047)
}
