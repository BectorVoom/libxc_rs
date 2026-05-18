//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 856/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk856<F: Float>(t15768: F, t3506: F, t15763: F, t3499: F, t16712: F, t2102: F, t15752: F, t15756: F, t363: F, t4822: F, t12796: F, t2112: F, t358: F) -> (F, F, F, F, F, F, F, F) {
    let t17319 = t3506 * t15768;
    let t17322 = t3499 * t15763;
    let t17325 = t2102 * t16712;
    let t17328 = t3506 * t15752;
    let t17331 = t3506 * t15756;
    let t17334 = t4822 * t363;
    let t17335 = t12796 * t17334;
    let t17338 = t2112 * t358;
    (t17319, t17322, t17325, t17328, t17331, t17334, t17335, t17338)
}
