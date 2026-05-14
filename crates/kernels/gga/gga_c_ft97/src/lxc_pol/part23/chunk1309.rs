//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1309/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1309<F: Float>(t55797: F, t7114: F, t10409: F, t10683: F, t111747: F, t111751: F, t111795: F, t111801: F, t25446: F, t25459: F, t25465: F, t2665: F, t29017: F, t29024: F, t31352: F, t31963: F, t4162: F, t4965: F, t4973: F, t6216: F, t6263: F, t6963: F, t98306: F, t98309: F) -> (F, F) {
    let t125586 = t55797 * t7114;
    let t125608 = t111747 + 8.0 * t125586 + t31963 * t6263 / 6.0 - t25459 * t31352 / 27.0 - t6216 * t10409 * t25465 * t4965 / 27.0 - t6216 * t2665 * t25446 * t4973 / 18.0 + t6963 * t29017 / 3.0 + 2.0 / 27.0 * t98306 + 2.0 / 27.0 * t98309 - t111751 - t111795 + 2.0 * t6216 * t10683 * t29024 * t4162 - t111801;
    (t125586, t125608)
}
