//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1342/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1342<F: Float>(t113397: F, t113398: F, t113400: F, t113402: F, t113404: F, t113406: F, t113408: F, t113410: F, t113412: F, t113414: F, t113416: F, t113443: F, t113488: F, t113489: F, t113491: F, t113493: F, t113495: F, t113498: F, t113500: F, t113502: F, t113504: F, t113506: F, t113508: F, t113533: F, t1459: F) -> (F,) {
    let t113537 = t1459 * (t113397 - 2.0 / 3.0 * t113398 - t113400 / 12.0 - t113402 / 48.0 + 2.0 / 9.0 * t113404 - t113406 / 36.0 - t113408 / 32.0 + t113410 / 9.0 - 19.0 / 72.0 * t113412 - t113414 / 48.0 + 11.0 / 18.0 * t113416 + t113443 + t113488 - t113489 / 24.0 + t113491 / 3.0 + t113493 / 12.0 - t113495 / 24.0 + t113498 / 8.0 + t113500 / 12.0 + t113502 / 24.0 + t113504 / 64.0 - t113506 / 9.0 + t113508 / 128.0 + t113533);
    (t113537,)
}
