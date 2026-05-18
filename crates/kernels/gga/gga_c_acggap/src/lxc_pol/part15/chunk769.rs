//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 769/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk769<F: Float>(t1318: F, t7815: F, t2030: F, t1545: F, t7561: F, t1549: F, t7822: F, t1554: F, t1558: F, t1421: F, t599: F, t1181: F) -> (F, F, F, F, F, F, F, F) {
    let t8637 = t7815 * t1318;
    let t8638 = t2030 * t8637;
    let t8640 = t7561 * t1545;
    let t8642 = t7822 * t1549;
    let t8644 = t7822 * t1554;
    let t8646 = t7822 * t1558;
    let t8648 = t599 * t1421;
    let t8649 = t1181 * t8648;
    (t8637, t8638, t8640, t8642, t8644, t8646, t8648, t8649)
}
