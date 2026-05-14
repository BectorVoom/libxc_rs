//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1268/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1268<F: Float>(t3174: F, t9355: F, t982: F, t12659: F, t3241: F, t1035: F, t12769: F, t167: F, t3253: F, t12674: F, t9345: F, t12693: F, t31831: F, t3139: F, t967: F, t31834: F, t3236: F) -> (F, F, F, F, F, F, F, F) {
    let t110840 = t982 * t9355 * t3174;
    let t110842 = t3241 * t12659;
    let t110845 = t1035 * t167 * t12769;
    let t110847 = t3253 * t9355;
    let t110849 = t12674 * t9345;
    let t110851 = t12693 * t31831;
    let t110854 = t3241 * t967 * t3139;
    let t110856 = t3236 * t31834;
    (t110840, t110842, t110845, t110847, t110849, t110851, t110854, t110856)
}
