//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 278/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk278<F: Float>(t1023: F, t1054: F, t1058: F, t1060: F, t149: F, t165: F, t184: F, t632: F, t72: F, t920: F, t1002: F, t641: F, t927: F) -> (F, F, F, F) {
    let t1063 = -t1023 * t165 - t1058 * t149 - F::new(2.0) * t1054 + F::new(2.0) * t1060;
    let t1064 = t1063 * t184;
    let t1068 = t72 * t632 * t920;
    let t1073 = F::new(0.234754e0) * t1002 - t641 - F::new(0.14443083333333333333e0) * t927;
    (t1063, t1064, t1068, t1073)
}
