//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1024/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1024(t4454: f64, t4495: f64, t1800: f64, t24: f64, t3127: f64, t38508: f64, t462: f64, t469: f64, t58140: f64, t59078: f64, t59102: f64, t59104: f64, t59143: f64, t74266: f64, t74268: f64, t74285: f64, t74287: f64, t8327: f64, t85491: f64, t85682: f64, t85687: f64, t85692: f64, t92: f64) -> (f64, f64) {
    let t86161 = t4454 * t4495;
    let t86168 = 4.0_f64 / 3.0_f64 * t74266 + 8.0_f64 * t74268 - t92 * t24 * t469 * t85682 + 24.0_f64 * t92 * t24 * t38508 * t85692 + 6.0_f64 * t92 * t24 * t1800 * t85687 - 8.0_f64 / 3.0_f64 * t58140 + 16.0_f64 / 3.0_f64 * t59078 - 8.0_f64 * t74285 - 16.0_f64 / 9.0_f64 * t74287 + 8.0_f64 * t462 * t3127 * t85491 + 4.0_f64 / 3.0_f64 * t462 * t8327 * t86161 + 16.0_f64 / 9.0_f64 * t59102 - 16.0_f64 / 27.0_f64 * t59104 - 8.0_f64 / 9.0_f64 * t59143;
    (t86161, t86168)
}
