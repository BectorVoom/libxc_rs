//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1068/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1068<F: Float>(t157: F, t1713: F, t406: F, t1165: F, t30282: F, t604: F, t30371: F, t5749: F, t2297: F, t8402: F, t13299: F, t31195: F) -> (F, F, F, F, F) {
    let t38843 = t1713 * t406 * t157;
    let t38846 = t30282 * t1165 * t604 * t38843;
    let t38848 = t30371 * t5749;
    let t38850 = t2297 * t8402;
    let t38852 = t31195 * t13299 * t38850;
    (t38843, t38846, t38848, t38850, t38852)
}
