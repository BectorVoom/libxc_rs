//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1155/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1155<F: Float>(t29721: F, t379: F, t38262: F, t446: F, t116281: F, t7824: F, t16011: F, t22986: F, t7793: F, t16155: F, t38268: F, t29717: F, t376: F, t5665: F, t29646: F, t93506: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t116418 = t29721 * t379;
    let t116420 = t446 * t38262 * t116418;
    let t116423 = t446 * t7824 * t116281;
    let t116425 = t22986 * t16011;
    let t116427 = t446 * t7793 * t116425;
    let t116429 = t22986 * t16155;
    let t116431 = t446 * t38268 * t116429;
    let t116434 = t5665 * t376 * t29717;
    let t116435 = t116434 / 6.0;
    let t116436 = t93506 * t29646;
    (t116418, t116420, t116423, t116425, t116427, t116429, t116431, t116434, t116435, t116436)
}
