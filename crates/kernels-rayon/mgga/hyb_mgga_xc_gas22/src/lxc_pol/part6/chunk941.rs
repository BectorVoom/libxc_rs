//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 941/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk941(t3287: f64, t550: f64, t136: f64, t1318: f64, t2152: f64, t26: f64, t3273: f64, t1292: f64, t1320: f64, t2004: f64, t3274: f64, t6227: f64, t6230: f64, t6425: f64, t6468: f64, t6483: f64, t6485: f64, t677: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8532 = t550 * t3287;
    let t8534 = t136 * t8532 / 32.0_f64;
    let t8535 = t2152 * t1318;
    let t8536 = t26 * t8535;
    let t8545 = t550 * t3273;
    let t8547 = t136 * t8545 / 32.0_f64;
    let t8553 = -t8534 - 3.0_f64 / 64.0_f64 * t136 * t8536 - 3.0_f64 / 64.0_f64 * t2004 * t1320 - 3.0_f64 / 64.0_f64 * t2004 * t1292 - 3.0_f64 / 32.0_f64 * t677 * t3274 - t8547 - t6425 / 32.0_f64 - t6468 - t6483 / 64.0_f64 + t6485 / 48.0_f64 + t6227 / 48.0_f64 - t6230 / 64.0_f64;
    (t8532, t8534, t8535, t8536, t8545, t8547, t8553)
}
