//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 697/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk697<F: Float>(t28719: F, t319: F, t840: F, t25271: F, t4176: F, t15460: F, t191: F, t295: F, t309: F, t10696: F, t1501: F, t4181: F) -> (F, F, F, F, F, F, F) {
    let t29120 = t840 * t319 * t28719;
    let t29123 = t25271 * t4176;
    let t29124 = t15460 * t29123;
    let t29127 = t191 * t295;
    let t29128 = t29127 * t309;
    let t29129 = t10696 * t1501;
    let t29130 = t29129 * t4181;
    (t29120, t29123, t29124, t29127, t29128, t29129, t29130)
}
