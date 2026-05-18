//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 861/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk861<F: Float>(t2012: F, t968: F, t177: F, t377: F, t7370: F, t2067: F, t3077: F, t7348: F, t4680: F, t7346: F, t7347: F, t1165: F, t16020: F, t604: F) -> (F, F, F, F, F, F) {
    let t30085 = t2012 * t968;
    let t30088 = t377 * t7370 * t177;
    let t30090 = t3077 * t2067;
    let t30091 = t30090 * t7348;
    let t30094 = t7346 * t4680 * t7347;
    let t30099 = t7346 * t1165 * t604 * t16020;
    (t30085, t30088, t30090, t30091, t30094, t30099)
}
