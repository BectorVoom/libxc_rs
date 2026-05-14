//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 541/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk541<F: Float>(t1060: F, t5136: F, t1857: F, t970: F, t1836: F, t960: F, t1843: F, t965: F, t167: F, t4597: F, t1797: F, t704: F, t1336: F, t140: F) -> (F, F, F, F, F, F, F) {
    let t5137 = t5136 * t1060;
    let t5142 = t970 * t1857;
    let t5150 = t960 * t1836;
    let t5158 = t965 * t1843;
    let t5168 = t167 * t4597;
    let t5180 = t1797 * t704;
    let t5182 = t140 * t1336 * t5180;
    (t5137, t5142, t5150, t5158, t5168, t5180, t5182)
}
