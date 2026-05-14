//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 952/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk952<F: Float>(t4393: F, t8511: F, t4414: F, t7822: F, t1181: F, t30327: F, t4358: F, t599: F, t30861: F, t8649: F, t4316: F, t4372: F, t7647: F, t1427: F, t1983: F, t34186: F, t7586: F) -> (F, F, F, F, F, F, F, F) {
    let t35232 = t8511 * t4393;
    let t35234 = t7822 * t4414;
    let t35238 = t30327 * t1181 * t599 * t4358;
    let t35240 = t30861 * t8649;
    let t35242 = t7822 * t4316;
    let t35244 = t7647 * t4372;
    let t35246 = t1983 * t1427;
    let t35248 = t34186 * t7586 * t35246;
    (t35232, t35234, t35238, t35240, t35242, t35244, t35246, t35248)
}
