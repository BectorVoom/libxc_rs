//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 817/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk817<F: Float>(t3608: F, t8272: F, t2274: F, t2436: F, t2435: F, t2432: F, t2554: F, t2352: F, t984: F, t92: F, t93: F, t352: F, t870: F, t301: F, t2329: F, t303: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t8273 = t3608 * t8272;
    let t8276 = t2436 * t2274;
    let t8277 = t2435 * t8276;
    let t8280 = t2554 * t2432;
    let t8283 = t984 * t2352;
    let t8285 = t92 * t92;
    let t8287 = 1.0 / t8285 * t93;
    let t8288 = t352 * t870;
    let t8289 = t301 * t301;
    let t8291 = t8287 * t8288 * t8289;
    let t8292 = t2329 * t303;
    (t8273, t8276, t8277, t8280, t8283, t8285, t8287, t8289, t8291, t8292)
}
