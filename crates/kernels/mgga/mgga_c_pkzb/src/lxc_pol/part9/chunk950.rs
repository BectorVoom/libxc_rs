//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 950/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk950<F: Float>(t237: F, t8112: F, t8148: F, t8188: F, t8238: F, t8041: F, t179: F, t2405: F, t3026: F, t404: F, t7945: F, t932: F, t1227: F, t931: F, t300: F) -> (F, F, F, F, F, F) {
    let t8241 = t237 * (t8112 + t8148 + t8188 + t8238);
    let t8243 = 0.19751673498613801407e-1 * t237 * t8041;
    let t8245 = t179 * t2405 * t3026;
    let t8247 = 0.57165357490759649296e-3 * t404 * t8245;
    let t8249 = t179 * t932 * t7945;
    let t8253 = t931 * t1227;
    let t8254 = t300 * t8253;
    (t8241, t8243, t8247, t8249, t8253, t8254)
}
