//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 700/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk700<F: Float>(t1255: F, t6260: F, t840: F, t7131: F, t824: F, t1508: F, t4129: F, t1212: F, t6393: F, t684: F, t7105: F, t10703: F) -> (F, F, F, F, F, F) {
    let t29170 = t840 * t1255 * t6260;
    let t29174 = t840 * t7131 * t824;
    let t29178 = t840 * t1508 * t4129;
    let t29182 = t840 * t6393 * t1212;
    let t29185 = t7105 * t684;
    let t29186 = t10703 * t29185;
    (t29170, t29174, t29178, t29182, t29185, t29186)
}
