//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 799/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk799<F: Float>(t2410: F, t4443: F, t674: F, t7411: F, t7288: F, t8659: F, t2286: F, t7921: F, t14249: F, t16503: F, t559: F, t7482: F, t16504: F, t2318: F, t34975: F, t7467: F) -> (F, F, F, F, F) {
    let t39892 = t2410 * t4443 * t674;
    let t39893 = t39892 * t7411;
    let t39899 = t8659 * t7288;
    let t39901 = t7921 * t2286;
    let t39907 = t16503 * t14249 * t559 * t7482;
    let t39911 = t34975 * t16504 * t2318 * t7467;
    (t39893, t39899, t39901, t39907, t39911)
}
