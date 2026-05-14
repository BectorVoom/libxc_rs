//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1191/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1191<F: Float>(t101587: F, t1767: F, t23009: F, t28: F, t469: F, t6454: F, t1317: F, t26002: F, t376: F, t1637: F, t5665: F, t6496: F, t100382: F, t3281: F, t7824: F, t1800: F, t24: F) -> (F, F, F, F, F, F, F) {
    let t101588 = t101587 / 18.0;
    let t101592 = t23009 * t28 * t469 * t6454 * t1767;
    let t101595 = t1317 * t376 * t26002;
    let t101596 = 2.0 / 9.0 * t101595;
    let t101598 = t5665 * t1637 * t6496;
    let t101601 = t3281 * t7824 * t100382;
    let t101603 = t24 * t1800;
    (t101588, t101592, t101595, t101596, t101598, t101601, t101603)
}
