//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1151/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1151<F: Float>(t28: F, t3157: F, t469: F, t5665: F, t6454: F, t1307: F, t16462: F, t3103: F, t1317: F, t1800: F, t4533: F, t5617: F, t1564: F, t22993: F, t4431: F, t446: F) -> (F, F, F, F, F, F) {
    let t116358 = t5665 * t28 * t469 * t6454 * t3157;
    let t116363 = t5665 * t28 * t469 * t1307 * t16462;
    let t116365 = t6454 * t3103;
    let t116368 = t1317 * t28 * t1800 * t116365;
    let t116373 = t5665 * t28 * t469 * t5617 * t4533;
    let t116377 = t446 * t1564 * t22993 * t4431;
    (t116358, t116363, t116365, t116368, t116373, t116377)
}
