//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 982/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk982<F: Float>(t3325: F, t7766: F, t2189: F, t3331: F, t10498: F, t1203: F, t3330: F, t3481: F, t3227: F, t3444: F, t1169: F, t982: F) -> (F, F, F, F, F, F, F, F, F) {
    let t26879 = 2.0 * t3325 * t7766;
    let t26880 = t2189 * t3331;
    let t26882 = 6.0 * t10498 * t26880;
    let t26883 = t7766 * t1203;
    let t26885 = 4.0 * t3330 * t26883;
    let t26886 = t2189 * t3481;
    let t26888 = 2.0 * t3330 * t26886;
    let t26889 = t3227 * t3444;
    let t26891 = t1169 * t982;
    (t26879, t26880, t26882, t26883, t26885, t26886, t26888, t26889, t26891)
}
