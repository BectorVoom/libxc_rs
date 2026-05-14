//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 790/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk790<F: Float>(t5857: F, t8392: F, t160: F, t5842: F, t1882: F, t5882: F, t91: F, t9252: F, t26: F, t376: F, t5890: F, t5892: F, t1369: F, t5905: F, t1368: F, t458: F) -> (F, F, F, F, F, F, F, F) {
    let t23576 = t8392 * t5857;
    let t23581 = t160 * t5842;
    let t23598 = t1882 * t5882;
    let t23608 = t91 * t9252;
    let t23609 = t23608 * t26;
    let t23616 = t5890 * t376 * t5892;
    let t23629 = t1369 * t376 * t5905;
    let t23649 = t1368 * t458;
    (t23576, t23581, t23598, t23608, t23609, t23616, t23629, t23649)
}
