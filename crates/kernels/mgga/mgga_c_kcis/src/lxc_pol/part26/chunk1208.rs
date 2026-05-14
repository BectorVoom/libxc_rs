//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1208/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1208<F: Float>(t16937: F, t29283: F, t27369: F, t1385: F, t27356: F, t5709: F, t6284: F, t12147: F, t29313: F, t7908: F, t28549: F, t94228: F, t98240: F, t1444: F, t1943: F, t5654: F, t98359: F) -> (F, F, F, F, F, F) {
    let t103239 = t16937 * t29283;
    let t103240 = t27369 * t103239;
    let t103251 = t5709 * t27356 * t6284 * t1385;
    let t103255 = t7908 * t12147 * t29313;
    let t103258 = t94228 * t98240 * t28549;
    let t103263 = t98359 * t1943 * t1444 * t5654;
    (t103239, t103240, t103251, t103255, t103258, t103263)
}
