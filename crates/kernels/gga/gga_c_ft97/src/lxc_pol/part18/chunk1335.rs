//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1335/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1335<F: Float>(t105733: F, t105703: F, t105708: F, t105712: F, t105715: F, t105720: F, t105722: F, t105725: F, t105730: F, t95225: F, t95228: F, t96092: F, t27124: F, t376: F, t5890: F, t1369: F, t27053: F) -> (F, F, F, F) {
    let t105734 = 4.0 / 9.0 * t105733;
    let t105737 = -4.0 / 3.0 * t105703 + 2.0 * t105708 + t105712 + t105715 / 3.0 + t105720 - 4.0 / 3.0 * t105722 - 8.0 / 3.0 * t105725 - t96092 - t105730 / 2.0 - t105734 + 8.0 / 9.0 * t95225 + 2.0 / 9.0 * t95228;
    let t105740 = t5890 * t376 * t27124;
    let t105741 = t105740 / 6.0;
    let t105743 = t1369 * t376 * t27053;
    (t105737, t105740, t105741, t105743)
}
