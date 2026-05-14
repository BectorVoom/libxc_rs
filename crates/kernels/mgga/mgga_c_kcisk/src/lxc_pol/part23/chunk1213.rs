//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1213/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1213<F: Float>(t1406: F, t2236: F, t415: F, t1451: F, t2213: F, t2232: F, t32058: F, t3748: F, t9815: F, t1286: F, t20634: F, t32045: F, t1411: F, t5975: F, t9469: F, t3491: F, t9800: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t33492 = t1406 * t2236;
    let t33493 = t415 * t33492;
    let t33495 = t2213 * t1451;
    let t33496 = t415 * t33495;
    let t33498 = t32058 * t2232;
    let t33499 = t415 * t33498;
    let t33501 = t3748 * t9815;
    let t33508 = t20634 * t1286;
    let t33509 = t32045 * t33508;
    let t33510 = t1411 * t33509;
    let t33512 = t9469 * t5975;
    let t33513 = t415 * t33512;
    let t33515 = t3491 * t9800;
    (t33492, t33493, t33495, t33496, t33498, t33499, t33501, t33508, t33509, t33510, t33512, t33513, t33515)
}
