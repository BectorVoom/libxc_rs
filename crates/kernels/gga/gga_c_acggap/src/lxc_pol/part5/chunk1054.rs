//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1054/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1054<F: Float>(t1137: F, t6290: F, t1140: F, t6294: F, t1131: F, t1150: F, t1313: F, t1524: F, t1567: F, t1884: F, t3282: F, t335: F, t4099: F, t4582: F, t4586: F, t4593: F, t513: F, t5235: F, t5906: F, t6300: F, t6304: F, t6388: F, t960: F) -> (F,) {
    let t21230 = t1137 * t6290;
    let t21232 = t1140 * t6294;
    let t21257 = -t1150 * t960 * t1884 * t1131 / 16.0 + t1150 * t960 * t1313 * t4099 / 8.0 + t335 * t4593 * t4586 / 24.0 - 7.0 / 72.0 * t21230 - 7.0 / 72.0 * t21232 + t335 * t4593 * t4582 / 12.0 + t1150 * t3282 * t6388 / 8.0 + t335 * t3282 * t5906 / 24.0 + t335 * t3282 * t6300 / 12.0 + t335 * t3282 * t6304 / 12.0 + t335 * t960 * t5235 * t513 / 24.0 + t335 * t960 * t1567 * t1524 / 12.0;
    (t21257,)
}
