//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1257/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1257<F: Float>(t110920: F, t111276: F, t111278: F, t111282: F, t111286: F, t111288: F, t111290: F, t111294: F, t111297: F, t111301: F, t111304: F, t31917: F, t9315: F, t31924: F, t9307: F, t31905: F, t31910: F) -> (F, F, F, F) {
    let t111306 = 0.56291666666666666668e-1 * t111276 + 0.62500000000000000002e-1 * t111278 - 0.24125000000000000001e-1 * t111282 - 0.62500000000000000002e-1 * t111286 - 0.120625e-1 * t111288 - 0.120625e-1 * t111290 - 0.10416666666666666667e-1 * t111294 - 0.24305555555555555556e0 * t111297 - 0.24305555555555555556e0 * t111301 + 0.59694999999999999999e-1 * t110920 - 0.69841875000000000003e-2 * t111304;
    let t111308 = t9315 * t31917;
    let t111310 = t31924 * t9307;
    let t111312 = t31905 * t31910;
    (t111306, t111308, t111310, t111312)
}
