//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1085/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1085<F: Float>(t2178: F, t6685: F, t23405: F, t26815: F, t1347: F, t1900: F, t7149: F, t40424: F, t5773: F, t1349: F, t26792: F, t376: F, t24116: F, t6580: F, t26560: F, t26569: F) -> (F, F, F, F, F, F, F, F) {
    let t104462 = t6685 * t2178;
    let t104474 = 2.0 / 3.0 * t23405 * t26815;
    let t104477 = t1347 * t7149 * t1900;
    let t104478 = t40424 * t5773;
    let t104484 = 2.0 / 9.0 * t1349 * t376 * t26792;
    let t104512 = t6580 * t24116;
    let t104519 = t1349 * t376 * t26560 / 9.0;
    let t104532 = t23405 * t26569 / 27.0;
    (t104462, t104474, t104477, t104478, t104484, t104512, t104519, t104532)
}
