//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 840/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk840<F: Float>(t38530: F, t9165: F, t40771: F, t8457: F, t36596: F, t9831: F, t36457: F, t9835: F, t1469: F, t3369: F, t39851: F, t559: F, t2412: F, t8582: F, t2191: F, t9790: F) -> (F, F, F, F, F, F, F) {
    let t45825 = t38530 * t9165;
    let t45827 = t40771 * t8457;
    let t45830 = t36596 * t9831;
    let t45832 = t36457 * t9835;
    let t45836 = t39851 * t3369 * t559 * t1469;
    let t45844 = t2412 * t8582;
    let t45846 = t2191 * t9790;
    (t45825, t45827, t45830, t45832, t45836, t45844, t45846)
}
