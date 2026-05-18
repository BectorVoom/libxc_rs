//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 843/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk843<F: Float>(t4714: F, t558: F, t167: F, t2185: F, t4829: F, t8392: F, t1053: F, t12277: F, t144: F, t3590: F, t569: F, t925: F) -> (F, F, F, F, F) {
    let t17099 = t4714 * t558;
    let t17101 = t2185 * t167 * t17099;
    let t17104 = t8392 * t4829;
    let t17106 = t12277 * t1053;
    let t17107 = t144 * t17106;
    let t17111 = t569 * t3590 * t925;
    (t17101, t17104, t17106, t17107, t17111)
}
