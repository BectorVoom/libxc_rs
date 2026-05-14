//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1332/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1332<F: Float>(t32269: F, t6357: F, t1415: F, t20983: F, t113348: F, t113351: F, t113353: F, t113356: F, t113359: F, t113362: F, t113365: F, t113367: F, t113370: F, t113373: F, t113376: F, t113379: F, t113381: F, t113383: F, t113385: F, t113387: F, t113389: F, t113391: F) -> (F, F, F) {
    let t113393 = t32269 * t6357;
    let t113395 = t1415 * t20983;
    let t113397 = t113348 / 288.0 + t113351 / 48.0 + t113353 / 96.0 - t113356 / 16.0 + t113359 / 27.0 - t113362 / 32.0 + 3.0 / 64.0 * t113365 - t113367 / 64.0 - 2.0 / 9.0 * t113370 - t113373 / 144.0 + t113376 / 12.0 + t113379 / 6.0 + t113381 / 24.0 - t113383 / 96.0 + t113385 / 8.0 + 2.0 / 27.0 * t113387 - t113389 / 72.0 - t113391 / 12.0 - t113393 / 12.0 - t113395 / 12.0;
    (t113393, t113395, t113397)
}
