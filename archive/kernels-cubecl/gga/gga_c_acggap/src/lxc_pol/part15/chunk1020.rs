//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1020/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1020<F: Float>(t7433: F, t8908: F, t8912: F, t7346: F, t7347: F, t8480: F, t7447: F, t8823: F, t7440: F, t8826: F, t30817: F, t8948: F) -> (F, F, F, F, F, F) {
    let t35835 = t7433 * t8908;
    let t35837 = t7433 * t8912;
    let t35844 = t7346 * t8480 * t7347;
    let t35848 = t7447 * t8823;
    let t35850 = t7440 * t8826;
    let t35874 = t30817 * t8948;
    (t35835, t35837, t35844, t35848, t35850, t35874)
}
