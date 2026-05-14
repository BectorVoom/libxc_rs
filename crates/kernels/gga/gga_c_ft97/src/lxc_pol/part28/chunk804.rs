//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 804/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk804<F: Float>(t19: F, t7: F, t11: F, t1690: F, t76: F, t8050: F, t378: F, t7241: F, t1586: F, t1642: F, t22: F, t36452: F, t96: F, t1554: F, t355: F, t102: F, t8416: F) -> (F, F, F, F, F, F, F, F, F) {
    let t37991 = t7 * t19;
    let t38176 = t1690 * t11;
    let t38241 = 1.0 / t8050 / t76;
    let t38262 = t378 * t7241;
    let t38268 = t1642 * t1586;
    let t38456 = 1.0 / t96 / t37991 / t22 / t1586 / t36452 / 96.0;
    let t38463 = t1554 * t1586;
    let t38477 = t355 * t7241;
    let t38651 = 1.0 / t8416 / t102;
    (t37991, t38176, t38241, t38262, t38268, t38456, t38463, t38477, t38651)
}
