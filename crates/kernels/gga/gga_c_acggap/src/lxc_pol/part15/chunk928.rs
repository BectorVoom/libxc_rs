//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 928/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk928<F: Float>(t2113: F, t7610: F, t2082: F, t30567: F, t7528: F, t7637: F, t2109: F, t1101: F, t1983: F, t30827: F, t7586: F, t3378: F, t7584: F) -> (F, F, F, F, F, F) {
    let t31660 = t7610 * t2113;
    let t31662 = t30567 * t2082;
    let t31682 = t7637 * t7528;
    let t31684 = t7610 * t2109;
    let t31693 = t30827 * t7586 * t1983 * t1101;
    let t31699 = t3378 * t7584;
    (t31660, t31662, t31682, t31684, t31693, t31699)
}
