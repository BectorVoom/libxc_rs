//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 915/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk915<F: Float>(t1543: F, t7332: F, t22685: F, t22687: F, t22689: F, t22692: F, t22695: F, t22697: F, t22701: F, t22703: F, t22705: F, t22707: F, t22320: F, t22322: F, t22325: F, t22327: F, t22330: F, t22333: F, t22336: F, t22339: F, t22343: F, t22345: F, t22379: F, t22382: F, t22385: F, t22388: F, t22391: F, t22394: F, t22396: F, t22398: F, t22401: F, t22404: F, t22407: F, t22439: F, t22442: F, t22444: F, t22447: F, t22449: F, t22453: F, t22457: F, t22459: F, t22461: F, t22464: F, t22467: F, t22657: F, t22659: F, t22662: F, t22666: F, t22668: F, t22670: F, t22672: F, t22675: F, t22677: F, t22679: F, t22682: F) -> (F, F) {
    let t22709 = t1543 * t7332;
    let t22711 = -t22685 / 16.0 - t22687 / 24.0 - t22689 / 96.0 + t22692 / 256.0 + 2.0 / 27.0 * t22695 - t22697 / 3.0 - t22701 / 16.0 - t22703 / 192.0 - t22705 / 192.0 + 2.0 / 9.0 * t22707 + t22709 / 18.0;
    let t22714 = t22467 / 4.0 + t22339 / 864.0 + t22453 / 256.0 + 2.0 / 9.0 * t22670 + t22672 / 24.0 + t22333 / 12.0 + t22336 / 96.0 - t22385 / 12.0 - t22388 / 96.0 - t22675 / 48.0 + t22677 / 3.0 + t22679 / 18.0 - 3.0 / 8.0 * t22330 - 2.0 / 3.0 * t22682 - t22391 / 36.0 + t22379 - t22457 / 192.0 - t22459 / 576.0 + t22343 / 36.0 - 2.0 / 9.0 * t22345 + t22407 / 72.0 - 11.0 / 18.0 * t22447 - 2.0 / 9.0 * t22449 + t22666 / 6.0 + t22668 / 128.0 + t22398 / 12.0 + t22401 / 54.0 + t22444 / 96.0 + 11.0 / 18.0 * t22320 - 19.0 / 144.0 * t22322 + t22382 / 576.0 + t22657 - t22659 / 24.0 - t22394 / 64.0 - t22396 / 12.0 + t22439 - t22442 / 12.0 - t22325 / 24.0 + t22327 / 96.0 + t22711 + t22461 / 24.0 - t22464 / 288.0 + t22662 / 3.0 - t22404 / 24.0;
    (t22709, t22714)
}
