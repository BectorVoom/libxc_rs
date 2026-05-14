//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1068/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1068<F: Float>(t2097: F, t2142: F, t582: F, t9439: F, t167: F, t40465: F, t2101: F, t3578: F, t616: F, t9114: F, t3539: F, t40424: F, t9276: F, t3099: F, t428: F, t401: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t50554 = t2097 * t2142;
    let t50558 = t582 * t9439;
    let t50744 = t40465 * t167;
    let t50773 = t2101 * t3578;
    let t51032 = t9114 * t616;
    let t51036 = t2101 * t3539;
    let t51151 = t40424 * t167;
    let t51170 = t582 * t9276;
    let t58524 = t428 * t3099;
    let t58531 = t401 * t3099;
    (t50554, t50558, t50744, t50773, t51032, t51036, t51151, t51170, t58524, t58531)
}
