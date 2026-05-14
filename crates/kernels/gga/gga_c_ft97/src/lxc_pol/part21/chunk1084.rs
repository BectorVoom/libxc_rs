//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1084/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1084<F: Float>(t614: F, t6615: F, t1349: F, t26514: F, t376: F, t23405: F, t26805: F, t26770: F, t26574: F, t5766: F, t1637: F, t6621: F, t26580: F, t92: F, t24073: F, t6580: F) -> (F, F, F, F, F, F, F, F) {
    let t104364 = t6615 * t614;
    let t104379 = t1349 * t376 * t26514 / 9.0;
    let t104426 = t23405 * t26805 / 27.0;
    let t104432 = t1349 * t376 * t26770 / 9.0;
    let t104434 = t5766 * t26574 / 9.0;
    let t104436 = t1349 * t1637 * t6621;
    let t104446 = t26580 * t92;
    let t104450 = 2.0 / 9.0 * t6580 * t24073;
    (t104364, t104379, t104426, t104432, t104434, t104436, t104446, t104450)
}
