//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1151/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1151<F: Float>(t32278: F, t4232: F, t4296: F, t488: F, t1506: F, t3508: F, t4313: F, t485: F, t1299: F, t9498: F, t3502: F, t4204: F, t9497: F, t12817: F, t500: F, t3732: F, t6332: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t32279 = t32278 * t4232;
    let t32281 = t4296 * t488;
    let t32283 = t3508 * t1506;
    let t32285 = t485 * t4313;
    let t32287 = t485 * t1299;
    let t32288 = t32287 * t9498;
    let t32290 = t4204 * t3502;
    let t32291 = t9497 * t32290;
    let t32293 = t12817 * t500;
    let t32295 = t6332 * t3732;
    (t32279, t32281, t32283, t32285, t32287, t32288, t32290, t32291, t32293, t32295)
}
