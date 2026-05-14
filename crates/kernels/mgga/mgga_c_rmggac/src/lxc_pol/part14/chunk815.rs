//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 815/952 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk815<F: Float>(t1686: F, t2039: F, t270: F, t638: F, t1692: F, t1338: F, t535: F, t2046: F, t2050: F, t31: F, t333: F, t3351: F, t511: F, t7248: F, t9216: F, t352: F, t515: F) -> (F, F, F, F, F, F) {
    let t39796 = t638 * t2039 * t1686 * t270;
    let t39797 = 0.30487649791575028314e-3 * t39796;
    let t39800 = t638 * t2039 * t1692 * t270;
    let t39801 = 0.30487649791575028314e-3 * t39800;
    let t39804 = t638 * t2039 * t535 * t1338;
    let t39808 = t2046 * t2050 * t1686 * t31;
    let t39809 = 0.43368970657079495312e-4 * t39808;
    let t39813 = t3351 * t7248 * t511 * t9216 * t333;
    let t39818 = t3351 * t7248 * t515 * t9216 * t352;
    (t39797, t39801, t39804, t39809, t39813, t39818)
}
