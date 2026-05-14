//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1335/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1335<F: Float>(t21269: F, t488: F, t21024: F, t32278: F, t21014: F, t33652: F, t21093: F, t13383: F, t2275: F, t113419: F, t113422: F, t113424: F, t113426: F, t113428: F, t113431: F, t1506: F, t19951: F) -> (F, F, F, F, F, F, F) {
    let t113433 = t21269 * t488;
    let t113435 = t32278 * t21024;
    let t113437 = t33652 * t21014;
    let t113439 = t32278 * t21093;
    let t113441 = t13383 * t2275;
    let t113443 = t113419 / 4.0 + t113422 / 36.0 - t113424 / 288.0 + t113426 / 96.0 + t113428 / 54.0 - t113431 / 8.0 + t113433 / 16.0 + t113435 / 48.0 - t113437 / 64.0 + t113439 / 144.0 + t113441 / 128.0;
    let t113446 = t19951 * t1506;
    (t113433, t113435, t113437, t113439, t113441, t113443, t113446)
}
