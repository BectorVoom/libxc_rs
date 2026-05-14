//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1152/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1152<F: Float>(t32295: F, t9491: F, t3777: F, t6317: F, t9497: F, t1513: F, t3512: F, t1517: F, t1340: F, t4193: F, t32279: F, t32281: F, t32283: F, t32285: F, t32288: F, t32291: F, t32293: F) -> (F, F, F, F, F, F, F) {
    let t32296 = t9491 * t32295;
    let t32298 = t6317 * t3777;
    let t32299 = t9497 * t32298;
    let t32301 = t3512 * t1513;
    let t32303 = t3512 * t1517;
    let t32305 = t1340 * t4193;
    let t32307 = t32279 / 48.0 + t32281 / 16.0 + t32283 / 64.0 + 11.0 / 18.0 * t32285 - 2.0 / 9.0 * t32288 + t32291 / 24.0 - t32293 / 128.0 - t32296 / 16.0 - t32299 / 72.0 + t32301 / 12.0 - t32303 / 48.0 - t32305 / 96.0;
    (t32296, t32298, t32299, t32301, t32303, t32305, t32307)
}
