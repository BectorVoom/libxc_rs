//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1540/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1540<F: Float>(t14164: F, t17686: F, t4582: F, t17691: F, t4583: F, t1023: F, t17670: F, t4594: F, t17167: F, t977: F, t17171: F, t17157: F, t2979: F) -> (F, F, F, F, F, F, F) {
    let t17971 = t14164 * t17686;
    let t17972 = t4582 * t17971;
    let t17975 = t4583 * t17691;
    let t17976 = t4582 * t17975;
    let t17979 = t17670 * t1023;
    let t17980 = t4582 * t17979;
    let t17983 = t17670 * t4594;
    let t17984 = t4582 * t17983;
    let t17988 = t977 * t17167;
    let t17991 = t977 * t17171;
    let t17994 = t2979 * t17157;
    (t17972, t17976, t17980, t17984, t17988, t17991, t17994)
}
