//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 863/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk863<F: Float>(t2130: F, t3035: F, t3357: F, t7741: F, t3243: F, t597: F, t2100: F, t7538: F, t7544: F, t1004: F, t1979: F, t7548: F) -> (F, F, F, F, F, F, F) {
    let t30032 = t3035 * t2130;
    let t30037 = t7741 * t3357;
    let t30044 = t3243 * t597;
    let t30045 = t30044 * t2100;
    let t30047 = t7538 * t7544;
    let t30049 = t1004 * t1979;
    let t30050 = t30049 * t7548;
    (t30032, t30037, t30044, t30045, t30047, t30049, t30050)
}
