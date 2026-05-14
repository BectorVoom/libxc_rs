//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1107/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1107<F: Float>(t19851: F, t546: F, t108: F, t2214: F, t122: F, t6327: F, t10979: F, t128: F, t113: F, t1391: F, t20: F, t6192: F, t1582: F, t2097: F, t571: F, t2111: F, t2117: F, t4888: F, t57: F) -> (F, F, F, F, F, F, F, F) {
    let t20338 = t546 * t19851;
    let t20407 = t2214 * t108;
    let t20420 = t6327 * t122;
    let t20421 = t10979 * t128;
    let t20422 = t20420 * t20421;
    let t20424 = t113 * t20 * t1391;
    let t20426 = t20422 * t6192 * t20424;
    let t20434 = t571 * t1582 * t2097;
    let t20445 = 0.14714292610726565554e-1 * t2111 * t122 * t4888 * t57 * t2117;
    (t20338, t20407, t20420, t20422, t20424, t20426, t20434, t20445)
}
