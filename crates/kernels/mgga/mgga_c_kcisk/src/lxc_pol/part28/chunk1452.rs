//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1452/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1452<F: Float>(t111201: F, t111203: F, t111206: F, t120905: F, t120906: F, t120907: F, t122154: F, t123470: F, t2356: F, t25290: F, t2819: F, t34008: F, t34657: F, t35536: F, t564: F, t567: F, t8464: F, t9639: F, t9642: F, t9777: F, t9904: F) -> (F,) {
    let t123479 = -t111201 + t120905 + t111203 + t120906 + t120907 + t111206 + t9904 * t34008 / 8.0 - t564 * t25290 * t2819 / 16.0 - t8464 * t9642 / 8.0 + t2356 * t34657 / 8.0 - t564 * t567 * (t122154 + t123470) / 16.0 - t8464 * t9777 / 8.0 - t35536 * t9639 / 8.0;
    (t123479,)
}
