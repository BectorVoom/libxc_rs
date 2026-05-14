//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 799/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk799<F: Float>(t30318: F, t438: F, t2092: F, t7610: F, t1165: F, t30209: F, t3655: F, t7351: F, t12935: F, t7336: F, t1181: F, t3355: F, t599: F, t604: F, t3044: F, t3809: F, t7493: F) -> (F, F, F, F, F, F, F, F) {
    let t30319 = t30318 * t438;
    let t30321 = t7610 * t2092;
    let t30325 = t30209 * t1165 * t7351 * t3655;
    let t30327 = t12935 * t7336;
    let t30330 = t30327 * t1181 * t599 * t3355;
    let t30334 = t30209 * t1181 * t604 * t3655;
    let t30339 = t30209 * t1181 * t599 * t3044;
    let t30340 = 0.64311027177104605458e-3 * t30339;
    let t30343 = t7493 * t1181 * t599 * t3809;
    (t30319, t30321, t30325, t30327, t30330, t30334, t30340, t30343)
}
