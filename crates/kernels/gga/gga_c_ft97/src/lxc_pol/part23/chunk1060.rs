//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1060/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1060<F: Float>(t299: F, t31661: F, t31971: F, t332: F, t113: F, t1275: F, t1512: F, t29429: F, t4635: F, t5: F, t5475: F, t5480: F, t5483: F, t6403: F, t7138: F, t992: F, t240: F, t7513: F) -> (F, F, F, F) {
    let t300 = 10000000.0 <= t299;
    let t31972 = t31661 + t31971;
    let t31973 = t31972 * t332;
    let t31992 = piecewise3(t300, 0.0, t5 * t31973 * t113 / 4.0 + t29429 * t1275 / 2.0 + t5 * t7138 * t992 / 2.0 + t6403 * t5475 / 4.0 + t6403 * t5480 / 4.0 + t6403 * t5483 / 2.0 + t5 * t1512 * t4635 / 4.0);
    let t33300 = 1.0 / t7513 / t240;
    (t31972, t31973, t31992, t33300)
}
