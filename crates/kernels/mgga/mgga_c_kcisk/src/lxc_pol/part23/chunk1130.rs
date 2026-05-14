//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1130/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1130<F: Float>(t1405: F, t1413: F, t1441: F, t415: F, t1406: F, t1451: F, t1292: F, t3930: F, t1308: F) -> (F, F, F, F, F, F, F) {
    let t32058 = t1405 * t1413;
    let t32059 = t32058 * t1441;
    let t32060 = t415 * t32059;
    let t32062 = t1406 * t1451;
    let t32063 = t415 * t32062;
    let t32065 = t3930 * t1292;
    let t32066 = t32065 * t1308;
    (t32058, t32059, t32060, t32062, t32063, t32065, t32066)
}
