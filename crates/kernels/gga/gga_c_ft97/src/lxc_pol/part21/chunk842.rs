//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 842/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk842<F: Float>(t5617: F, t942: F, t1800: F, t1317: F, t28: F, t1307: F, t3103: F, t473: F, t6454: F, t469: F, t5665: F, t3157: F, t965: F, t376: F, t6504: F, t6496: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t25996 = t5617 * t942;
    let t25997 = t1800 * t25996;
    let t25999 = t1317 * t28 * t25997;
    let t26001 = t1307 * t3103;
    let t26002 = t1800 * t26001;
    let t26004 = t1317 * t28 * t26002;
    let t26006 = t6454 * t473;
    let t26007 = t469 * t26006;
    let t26009 = t5665 * t28 * t26007;
    let t26011 = t1307 * t3157;
    let t26012 = t469 * t26011;
    let t26014 = t5665 * t28 * t26012;
    let t26016 = t5617 * t965;
    let t26017 = t469 * t26016;
    let t26019 = t5665 * t28 * t26017;
    let t26022 = t1317 * t376 * t6504;
    let t26025 = t5665 * t376 * t6496;
    (t25996, t25997, t25999, t26001, t26002, t26004, t26007, t26009, t26011, t26012, t26014, t26016, t26017, t26019, t26022, t26025)
}
