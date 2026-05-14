//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 815/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk815<F: Float>(t11135: F, t8862: F, t10802: F, t27229: F, t11969: F, t1960: F, t977: F, t24215: F, t3553: F, t13350: F, t4342: F, t41609: F, t41612: F, t41618: F, t41623: F, t41626: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t46013 = 4.0 * t8862 * t11135;
    let t46016 = 12.0 * t27229 * t10802;
    let t46019 = 2.0 * t1960 * t11969 * t977;
    let t46023 = 2.0 * t24215 * t3553;
    let t46025 = 2.0 * t4342 * t13350;
    let t46030 = 0.30674340763136599742e1 * t41609;
    let t46031 = 0.14570311862489884877e2 * t41612;
    let t46033 = 0.23833659967900284446e0 * t41618;
    let t46035 = 0.11916829983950142223e0 * t41623;
    let t46036 = 0.11916829983950142223e0 * t41626;
    (t46013, t46016, t46019, t46023, t46025, t46030, t46031, t46033, t46035, t46036)
}
