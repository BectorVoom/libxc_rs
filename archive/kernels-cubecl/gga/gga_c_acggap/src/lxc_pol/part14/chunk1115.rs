//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1115/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1115<F: Float>(t1095: F, t1980: F, t5659: F, t7476: F, t1089: F, t2079: F, t2080: F, t22099: F, t1165: F, t26995: F, t604: F, t7337: F) -> (F, F, F) {
    let t39438 = t1980 * t7476 * t1095 * t5659;
    let t39442 = t2079 * t1089 * t22099 * t2080;
    let t39447 = t7337 * t1165 * t604 * t26995;
    (t39438, t39442, t39447)
}
