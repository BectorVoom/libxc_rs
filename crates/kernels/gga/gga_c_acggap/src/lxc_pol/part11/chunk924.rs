//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 924/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk924<F: Float>(t1165: F, t4342: F, t7575: F, t8600: F, t1181: F, t30209: F, t4347: F, t604: F, t4402: F, t7822: F, t5094: F, t7564: F, t31878: F, t4925: F, t1541: F, t31631: F) -> (F, F, F, F, F, F) {
    let t34259 = t7575 * t1165 * t8600 * t4342;
    let t34263 = t30209 * t1181 * t604 * t4347;
    let t34265 = t7822 * t4402;
    let t34269 = t7564 * t1181 * t8600 * t5094;
    let t34271 = t31878 * t4925;
    let t34273 = t31631 * t1541;
    (t34259, t34263, t34265, t34269, t34271, t34273)
}
