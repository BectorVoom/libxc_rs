//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 777/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk777<F: Float>(t12: F, t24: F, t3366: F, t1151: F, t1153: F, t318: F, t319: F, t3706: F, t201: F, t398: F, t326: F, t2179: F, t3371: F, t3374: F, t821: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F, F, F, F) {
    let t84 = t12 <= zeta_threshold;
    let t90 = t24 <= zeta_threshold;
    let t203 = rho0 <= dens_threshold || t84;
    let t3710 = piecewise3(t84, 0.0, t3366);
    let t3714 = piecewise3(t203, 0.0, t3706 * t319 / 2.0 + t1151 * t1153 + t318 * t3710 / 2.0);
    let t3715 = t201 * t3714;
    let t3718 = 1.0 / t398;
    let t3719 = t326 * t3718;
    let t3725 = t2179 * t3371;
    let t3727 = t821 * t3374;
    let t3730 = piecewise3(t90, 0.0, 4.0 / 9.0 * t3725 - t3727 / 3.0);
    (t3710, t3715, t3719, t3725, t3727, t3730)
}
