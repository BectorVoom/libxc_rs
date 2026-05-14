//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 630/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk630<F: Float>(t12991: F, t12997: F, t12961: F, t12966: F, t12988: F, t12994: F, t13458: F, t13463: F, t13466: F, t13469: F, t13473: F, t13477: F, t574: F, t13382: F, t13418: F, t13456: F) -> (F,) {
    let t13478 = 0.59584149919750711116e-1 * t12991;
    let t13480 = 0.11916829983950142223e0 * t12997;
    let t13481 = -0.23005755572352449806e1 * t574 * t13458 + 0.38342925953920749677e1 * t12961 - 0.76685851907841499353e0 * t12966 - t13463 + 0.63904876589867916128e-1 * t12988 - 0.38342925953920749677e0 * t13466 - 0.57514388930881124515e0 * t13469 + t13473 + t13477 + t13478 + 0.76685851907841499353e0 * t12994 + t13480;
    let t13483 = t13382 + t13418 + t13456 + t13481;
    (t13483,)
}
