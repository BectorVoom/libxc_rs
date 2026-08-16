//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1851/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1851<F: Float>(t2091: F, t40590: F, t90500: F, t90511: F, t225: F, t27070: F, t27052: F, t90514: F, t90524: F, t90533: F, t90541: F, t90546: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t93319 = t40590 * t2091;
    let t93333 = F::cast_from(0.15352717957250113407e0_f64) * t90500;
    let t93337 = F::cast_from(0.15352717957250113407e0_f64) * t90511;
    let t93338 = t27070 * t225;
    let t93341 = t27052 * t225;
    let t93344 = F::cast_from(0.16449340668482264365e-1_f64) * t90514;
    let t93350 = F::cast_from(0.3289868133696452873e-1_f64) * t90524;
    let t93353 = F::cast_from(0.3289868133696452873e-1_f64) * t90533;
    let t93359 = F::cast_from(0.76763589786250567036e-1_f64) * t90541;
    let t93361 = F::cast_from(0.3289868133696452873e-1_f64) * t90546;
    (t93319, t93333, t93337, t93338, t93341, t93344, t93350, t93353, t93359, t93361)
}
