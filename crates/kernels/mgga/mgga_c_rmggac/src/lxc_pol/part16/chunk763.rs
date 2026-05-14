//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 763/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk763<F: Float>(t1341: F, t575: F, t638: F, t7310: F, t7244: F, t8427: F, t2001: F, t326: F, t498: F, t559: F, t40948: F, t903: F, t36733: F, t8450: F, t7478: F, t8432: F) -> (F, F, F, F, F, F, F) {
    let t42042 = t638 * t7310 * t575 * t1341;
    let t42044 = t7244 * t8427;
    let t42054 = t2001 * t326 * t559 * t498;
    let t42057 = t903 * t40948;
    let t42085 = t8450 * t36733;
    let t42086 = t42085 * t7478;
    let t42101 = t7244 * t8432;
    (t42042, t42044, t42054, t42057, t42085, t42086, t42101)
}
