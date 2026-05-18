//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 814/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk814<F: Float>(t2385: F, t315: F, t323: F, t157: F, t2217: F, t524: F, t2152: F, t119: F, t2387: F, t310: F, t557: F, t8331: F) -> (F, F, F, F, F, F) {
    let t9380 = t315 * t2385;
    let t9381 = t9380 * t323;
    let t9385 = t2217 * t524 * t157;
    let t9386 = t2152 * t9385;
    let t9391 = t119 * t2385;
    let t9397 = t310 * t2387;
    let t9399 = t8331 * t557;
    (t9380, t9381, t9386, t9391, t9397, t9399)
}
