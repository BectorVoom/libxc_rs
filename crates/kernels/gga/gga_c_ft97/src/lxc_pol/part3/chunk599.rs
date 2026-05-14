//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 599/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk599<F: Float>(t1771: F, t588: F, t2: F, t9114: F, t583: F, t8282: F, t9132: F, t143: F, t7760: F, t153: F, t525: F, t631: F, t637: F, t7242: F, t576: F, t8232: F) -> (F, F, F, F, F, F, F) {
    let t9179 = t1771 * t588;
    let t9192 = t9114 * t2;
    let t9202 = t8282 * t583;
    let t9217 = t9132 * t2;
    let t9224 = t7760 * t143;
    let t9252 = 1.0 / t153 / t631 / t637 / t525 / t7242 / 4.0;
    let t9270 = t8232 * t576;
    (t9179, t9192, t9202, t9217, t9224, t9252, t9270)
}
