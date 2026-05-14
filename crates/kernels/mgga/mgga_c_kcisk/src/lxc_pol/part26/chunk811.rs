//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 811/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk811<F: Float>(t12795: F, t143: F, t12425: F, t12435: F, t12499: F, t12630: F, t12734: F, t12786: F, t12789: F, t151: F, t175: F, t213: F, t2925: F, t3082: F, t3087: F, t3088: F, t3107: F, t3125: F, t60: F, t852: F, t945: F, t955: F, t972: F) -> (F,) {
    let t12796 = t143 * t12795;
    let t12809 = -0.70279601891642686494e-2 * t213 * t151 - 0.14055920378328537299e-1 * t12786 * t955 - 0.21083880567492805948e-1 * t12789 * t3088 + 0.70279601891642686494e-2 * t3082 * t3107 - 0.28111840756657074598e-1 * t12796 * t12435 + 0.21083880567492805948e-1 * t3087 * t12734 - 0.23426533963880895498e-2 * t945 * t12499 - t12630 * t175 - 3.0 * t2925 * t972 - 3.0 * t852 * t3125 - t60 * t12425;
    (t12809,)
}
