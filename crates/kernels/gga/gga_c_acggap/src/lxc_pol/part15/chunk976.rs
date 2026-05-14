//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 976/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk976<F: Float>(t368: F, t5655: F, t1980: F, t30058: F, t1165: F, t2068: F, t25727: F, t7351: F, t7337: F, t8480: F, t8774: F, t5727: F, t7647: F, t25742: F, t6271: F, t7561: F) -> (F, F, F, F, F, F, F) {
    let t39120 = t368 * t5655;
    let t39122 = t1980 * t30058 * t39120;
    let t39131 = t2068 * t1165 * t7351 * t25727;
    let t39134 = t7337 * t8480 * t8774;
    let t39136 = t7647 * t5727;
    let t39141 = t2068 * t1165 * t7351 * t25742;
    let t39143 = t7561 * t6271;
    (t39120, t39122, t39131, t39134, t39136, t39141, t39143)
}
