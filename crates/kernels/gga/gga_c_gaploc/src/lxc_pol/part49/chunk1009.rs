//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1009/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1009<F: Float>(t13821: F, t1628: F, t574: F, t13825: F, t597: F, t13750: F, t1589: F, t557: F, t13829: F, t193: F, t524: F, t1: F, t46873: F, t544: F, t1424: F, t42026: F, t42029: F, t42030: F, t42032: F, t48011: F, t48013: F, t48017: F) -> (F,) {
    let t48020 = 0.30674340763136599741e1 * t574 * t1628 * t13821;
    let t48023 = 0.30674340763136599741e1 * t597 * t1628 * t13825;
    let t48026 = 0.23833659967900284446e0 * t557 * t1589 * t13750;
    let t48029 = 0.35750489951850426669e0 * t524 * t13829 * t193;
    let t48032 = t544 * t46873 * t1;
    let t48034 = 0.39722766613167140743e-1 * t48032 * t1424;
    let t48037 = t48011 + t48013 + t48017 - t48020 + t48023 - t48026 + t48029 - 0.14896037479937677779e-1 * t42026 - t48034 + t42029 + 0.35750489951850426669e0 * t42030 + 0.35750489951850426669e0 * t42032;
    (t48037,)
}
