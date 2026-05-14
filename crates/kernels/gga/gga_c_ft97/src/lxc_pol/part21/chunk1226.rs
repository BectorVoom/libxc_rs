//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1226/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1226<F: Float>(t86: F, t113: F, t115130: F, t115169: F, t115208: F, t115248: F, t115273: F, t115306: F, t116112: F, t116138: F, t116176: F, t116217: F, t116249: F, t118367: F, t118402: F, t118434: F, t118462: F, t118485: F, t1342: F, t1577: F, t16579: F, t18: F, t26498: F, t30021: F, t4635: F, t5: F, t505: F, t5756: F, t6570: F, t992: F) -> (F,) {
    let t87 = 10000000.0 <= t86;
    let t118509 = piecewise3(t87, 0.0, t5 * (t115130 + t115169 + t115208 + t115248 + t115273 + t115306 + t116112 + t116138 + t116176 + t116217 + t116249 + t118367 + t118402 + t118434 + t118462 + t118485) * t113 / 4.0 + t5 * t30021 * t505 / 4.0 + t5 * t26498 * t992 / 2.0 - t5 * t6570 * t18 * t1577 + t5 * t5756 * t4635 / 4.0 + t5 * t1342 * t16579 / 4.0);
    (t118509,)
}
