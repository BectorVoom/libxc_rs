//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1048/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1048<F: Float>(t14477: F, t861: F, t141: F, t2464: F, t4579: F, t581: F, t2459: F, t2515: F, t14452: F, t835: F, t128: F) -> (F, F, F, F, F, F) {
    let t14478 = t861 * t14477;
    let t14479 = t141 * t14478;
    let t14481 = t2464 * t4579;
    let t14482 = t14481 * t581;
    let t14483 = t861 * t14482;
    let t14484 = t141 * t14483;
    let t14486 = t2459 * t4579;
    let t14487 = t14486 * t581;
    let t14488 = t2515 * t14487;
    let t14489 = t141 * t14488;
    let t14491 = t835 * t14452;
    let t14492 = t128 * t14491;
    (t14479, t14482, t14484, t14487, t14489, t14492)
}
