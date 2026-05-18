//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 865/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk865<F: Float>(t177: F, t2008: F, t980: F, t3646: F, t588: F, t377: F, t7370: F, t2067: F, t3077: F, t7348: F, t1160: F, t7432: F) -> (F, F, F, F, F, F) {
    let t30080 = t980 * t2008 * t177;
    let t30083 = t3646 * t588 * t177;
    let t30088 = t377 * t7370 * t177;
    let t30090 = t3077 * t2067;
    let t30091 = t30090 * t7348;
    let t30105 = t1160 * t7432;
    (t30080, t30083, t30088, t30090, t30091, t30105)
}
