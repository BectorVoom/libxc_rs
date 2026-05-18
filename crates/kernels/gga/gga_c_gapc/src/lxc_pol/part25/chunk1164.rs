//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1164/1444 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1164<F: Float>(t11808: F, t11983: F, t11772: F, t29692: F, t11795: F, t9387: F, t11508: F, t3402: F, t7944: F, t11513: F, t7259: F, t11822: F, t7511: F) -> (F, F, F, F, F, F) {
    let t33563 = t11808 * t11983;
    let t33565 = t11772 * t29692;
    let t33567 = t9387 * t11795;
    let t33570 = t3402 * t11508 * t7944;
    let t33576 = t7259 * t11513 * t7944;
    let t33578 = t11822 * t7511;
    (t33563, t33565, t33567, t33570, t33576, t33578)
}
