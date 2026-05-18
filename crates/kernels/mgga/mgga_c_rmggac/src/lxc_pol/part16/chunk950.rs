//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 950/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk950<F: Float>(t2286: F, t38351: F, t38530: F, t9171: F, t14243: F, t16503: F, t552: F, t9157: F, t9165: F, t40771: F, t8457: F, t36596: F, t9831: F) -> (F, F, F, F, F, F) {
    let t45813 = t38351 * t2286;
    let t45818 = t38530 * t9171;
    let t45822 = t16503 * t14243 * t552 * t9157;
    let t45825 = t38530 * t9165;
    let t45827 = t40771 * t8457;
    let t45830 = t36596 * t9831;
    (t45813, t45818, t45822, t45825, t45827, t45830)
}
