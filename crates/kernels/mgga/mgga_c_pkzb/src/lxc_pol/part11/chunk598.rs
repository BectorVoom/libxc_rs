//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 598/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk598<F: Float>(t12: F, t24: F, t124: F, t3380: F, t207: F, t3363: F, t3366: F, t652: F, t333: F, t3371: F, t3374: F, t821: F, zeta_threshold: F) -> (F, F) {
    let t84 = t12 <= zeta_threshold;
    let t90 = t24 <= zeta_threshold;
    let t3382 = 0.19751673498613801407e-1 * t3380 * t124;
    let t3388 = piecewise3(t84, 0.0, -2.0 / 9.0 * t652 * t3363 + 2.0 / 3.0 * t207 * t3366);
    let t3394 = piecewise3(t90, 0.0, -2.0 / 9.0 * t821 * t3371 + 2.0 / 3.0 * t333 * t3374);
    let t3396 = t3388 / 2.0 + t3394 / 2.0;
    (t3382, t3396)
}
