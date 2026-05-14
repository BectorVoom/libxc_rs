//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1004/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1004<F: Float>(t167: F, t40465: F, t2101: F, t3578: F, t3539: F, t40424: F, t582: F, t9276: F, t3157: F, t965: F, t7857: F, t929: F, t11120: F, t29482: F, t37940: F, t37487: F, t4441: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t50744 = t40465 * t167;
    let t50773 = t2101 * t3578;
    let t51036 = t2101 * t3539;
    let t51151 = t40424 * t167;
    let t51170 = t582 * t9276;
    let t57561 = t965 * t3157;
    let t58180 = t7857 * t929;
    let t58181 = t58180 * t11120;
    let t58185 = t29482 * t37940;
    let t58191 = t37487 * t4441 * t11120;
    (t50744, t50773, t51036, t51151, t51170, t57561, t58180, t58181, t58185, t58191)
}
