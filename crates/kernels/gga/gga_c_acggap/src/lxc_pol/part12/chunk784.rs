//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 784/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk784<F: Float>(t1980: F, t30058: F, t30059: F, t1998: F, t3732: F, t151: F, t177: F, t3558: F, t587: F, t2008: F, t980: F, t3646: F, t588: F, t2012: F, t968: F, t377: F, t7370: F) -> (F, F, F, F, F, F, F) {
    let t30061 = t1980 * t30058 * t30059;
    let t30073 = t1998 * t3732;
    let t30077 = t151 * t587 * t3558 * t177;
    let t30080 = t980 * t2008 * t177;
    let t30083 = t3646 * t588 * t177;
    let t30085 = t2012 * t968;
    let t30088 = t377 * t7370 * t177;
    (t30061, t30073, t30077, t30080, t30083, t30085, t30088)
}
