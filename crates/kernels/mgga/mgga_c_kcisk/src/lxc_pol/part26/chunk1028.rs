//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1028/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1028<F: Float>(t27272: F, t27304: F, t27344: F, t27393: F, t467: F, t488: F, t1497: F, t8233: F, t25966: F, t492: F, t500: F, t1512: F, t8278: F, t493: F, t2271: F, t6363: F, sigma0: F) -> (F, F, F, F, F, F, F, F) {
    let t27395 = t27272 + t27304 + t27344 + t27393;
    let t27396 = t27395 * t467;
    let t27397 = t27396 * sigma0;
    let t27398 = t27397 * t488;
    let t27400 = t8233 * t1497;
    let t27402 = t25966 * t467;
    let t27403 = t27402 * t492;
    let t27404 = t27403 * t500;
    let t27406 = t1512 * t8278;
    let t27407 = t493 * t27406;
    let t27409 = t2271 * t6363;
    (t27396, t27398, t27400, t27402, t27404, t27406, t27407, t27409)
}
