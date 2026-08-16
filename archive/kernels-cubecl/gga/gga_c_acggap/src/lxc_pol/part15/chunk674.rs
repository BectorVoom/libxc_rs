//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 674/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk674<F: Float>(t1165: F, t4210: F, t604: F, t7346: F, t587: F, t972: F, t151: F, t177: F, t2008: F, t377: F, t588: F, t980: F) -> (F, F, F, F, F, F) {
    let t7365 = t1165 * t604 * t4210;
    let t7366 = t7346 * t7365;
    let t7370 = t587 * t972;
    let t7372 = t151 * t7370 * t177;
    let t7375 = t377 * t2008 * t177;
    let t7378 = t980 * t588 * t177;
    (t7365, t7366, t7370, t7372, t7375, t7378)
}
