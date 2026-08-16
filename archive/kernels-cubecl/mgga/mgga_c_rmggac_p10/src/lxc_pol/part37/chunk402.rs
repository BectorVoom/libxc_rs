//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 402/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk402<F: Float>(t2127: F, t290: F, t1223: F, t28: F, t212: F, t672: F, t2084: F, t271: F, t2017: F, t262: F, t2016: F, t49: F, t639: F) -> (F, F, F, F, F) {
    let t7894 = t290 * t2127;
    let t7919 = t1223 * t28;
    let t7920 = t212 * t7919;
    let t7921 = t672 * t7920;
    let t7926 = t2084 * t271;
    let t7932 = t2017 * t262;
    let t7933 = t2016 * t7932;
    let t7934 = t639 * t49;
    (t7894, t7921, t7926, t7933, t7934)
}
