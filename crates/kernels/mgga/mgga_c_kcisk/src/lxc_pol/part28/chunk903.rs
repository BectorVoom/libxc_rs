//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 903/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk903<F: Float>(t1814: F, t220: F, t6743: F, t696: F, t1806: F, t6747: F, t6891: F, t970: F, t6894: F, t6928: F, t960: F, t6931: F, t2497: F, t3119: F, t2502: F, t3123: F) -> (F, F, F, F, F, F, F, F, F) {
    let t16117 = t1814 * t220;
    let t16122 = 0.93706135855523581992e-2 * t696 * t6743;
    let t16124 = 0.28111840756657074598e-1 * t1806 * t6747;
    let t16163 = 0.4705225e-4 * t970 * t6891;
    let t16167 = t970 * t6894;
    let t16188 = 0.18736e-1 * t960 * t6928;
    let t16190 = t960 * t6931;
    let t16204 = t3119 * t2497;
    let t16206 = t3123 * t2502;
    (t16117, t16122, t16124, t16163, t16167, t16188, t16190, t16204, t16206)
}
