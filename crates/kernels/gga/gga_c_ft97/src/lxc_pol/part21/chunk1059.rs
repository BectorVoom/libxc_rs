//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1059/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1059<F: Float>(t101587: F, t1317: F, t26002: F, t376: F, t1637: F, t5665: F, t6496: F, t1800: F, t24: F, t38477: F, t5675: F, t25982: F, t93506: F, t8270: F, t1636: F, t6520: F, t89: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t101588 = t101587 / 18.0;
    let t101595 = t1317 * t376 * t26002;
    let t101596 = 2.0 / 9.0 * t101595;
    let t101598 = t5665 * t1637 * t6496;
    let t101603 = t24 * t1800;
    let t101611 = t38477 * t5675;
    let t101615 = t93506 * t25982;
    let t101616 = t101615 / 54.0;
    let t101633 = t24 * t8270;
    let t101638 = t89 * t1636 * t6520;
    (t101588, t101595, t101596, t101598, t101603, t101611, t101615, t101616, t101633, t101638)
}
