//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 811/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk811<F: Float>(t157: F, t1984: F, t2097: F, t2179: F, t525: F, t1045: F, t9438: F, t1985: F, t597: F, t582: F, t9439: F, t167: F, t40465: F, t2101: F, t3578: F, t40424: F) -> (F, F, F, F, F, F, F, F, F) {
    let t50235 = t1984 * t157;
    let t50240 = t2097 * t2179;
    let t50249 = t525 * t157;
    let t50260 = t1045 * t9438;
    let t50268 = t1985 * t597;
    let t50558 = t582 * t9439;
    let t50744 = t40465 * t167;
    let t50773 = t2101 * t3578;
    let t51151 = t40424 * t167;
    (t50235, t50240, t50249, t50260, t50268, t50558, t50744, t50773, t51151)
}
