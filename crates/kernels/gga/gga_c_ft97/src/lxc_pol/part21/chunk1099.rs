//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1099/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1099<F: Float>(t2101: F, t5900: F, t1900: F, t6: F, t91: F, t9252: F, t23649: F, t27138: F, t105411: F, t105416: F, t105433: F, t105457: F, t105459: F, t105482: F, t105510: F, t105516: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t105909 = t2101 * t5900;
    let t105923 = t91 * t9252 * t6 * t1900;
    let t105941 = t23649 * t27138;
    let t105942 = t105941 / 9.0;
    let t105971 = t105411 / 27.0;
    let t105973 = t105416 / 9.0;
    let t105981 = t105433 / 54.0;
    let t105989 = 2.0 / 27.0 * t105457;
    let t105990 = 2.0 / 27.0 * t105459;
    let t105997 = 2.0 / 27.0 * t105482;
    let t106009 = 2.0 / 81.0 * t105510;
    let t106011 = 2.0 / 27.0 * t105516;
    (t105909, t105923, t105941, t105942, t105971, t105973, t105981, t105989, t105990, t105997, t106009, t106011)
}
