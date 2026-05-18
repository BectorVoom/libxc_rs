//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 865/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk865<F: Float>(t3350: F, t39207: F, t7751: F, t674: F, t7715: F, t8687: F, t1997: F, t7243: F, t8576: F, t1973: F, t16156: F, t9138: F) -> (F, F, F, F, F) {
    let t39277 = t39207 * t3350;
    let t39278 = t39277 * t7751;
    let t39281 = t8687 * t7715 * t674;
    let t39282 = t39281 * t1997;
    let t39284 = t8576 * t7243;
    let t39285 = t39284 * t1973;
    let t39289 = t16156 * t9138;
    (t39277, t39278, t39282, t39285, t39289)
}
