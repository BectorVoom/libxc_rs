//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 914/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk914<F: Float>(t3287: F, t550: F, t136: F, t1318: F, t2152: F, t26: F, t3273: F, t1292: F, t1320: F, t2004: F, t3274: F, t6227: F, t6230: F, t6425: F, t6468: F, t6483: F, t6485: F, t677: F) -> (F, F, F, F, F, F, F) {
    let t8532 = t550 * t3287;
    let t8534 = t136 * t8532 / 32.0;
    let t8535 = t2152 * t1318;
    let t8536 = t26 * t8535;
    let t8545 = t550 * t3273;
    let t8547 = t136 * t8545 / 32.0;
    let t8553 = -t8534 - 3.0 / 64.0 * t136 * t8536 - 3.0 / 64.0 * t2004 * t1320 - 3.0 / 64.0 * t2004 * t1292 - 3.0 / 32.0 * t677 * t3274 - t8547 - t6425 / 32.0 - t6468 - t6483 / 64.0 + t6485 / 48.0 + t6227 / 48.0 - t6230 / 64.0;
    (t8532, t8534, t8535, t8536, t8545, t8547, t8553)
}
