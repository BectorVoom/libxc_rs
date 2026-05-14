//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 565/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk565<F: Float>(t1597: F, t8403: F, t1557: F, t2332: F, t4519: F, t5880: F, t5972: F, t5979: F, t6426: F, t7834: F, t7837: F, t7840: F, t7909: F, t8075: F, t8289: F, t4347: F, t548: F, t5610: F, t8080: F, t8084: F, t8087: F, t8091: F, t8095: F, t8165: F, t8173: F, t8178: F, t8182: F, t8396: F) -> (F, F, F) {
    let t8404 = t8403 * t1597;
    let t8417 = 0.193e0 * t1557 * t8289 - 0.193e0 * t1557 * t8404 - 0.386e0 * t6426 * t2332 + 0.11607361111111111111e-2 * t7834 - 0.34822083333333333332e-2 * t7837 + 0.23214722222222222222e-2 * t7840 - 0.17411041666666666666e-2 * t7909 - t4519 + 0.23214722222222222222e-2 * t5880 - 0.23214722222222222222e-2 * t5972 + 0.15476481481481481481e-2 * t5979 + 0.34822083333333333332e-2 * t8075;
    let t8431 = -0.23214722222222222222e-2 * t8080 - 0.38691203703703703703e-3 * t8084 + 0.23214722222222222222e-2 * t8087 + 0.11607361111111111111e-2 * t8091 + 0.19345601851851851852e-2 * t8095 + 0.17411041666666666666e-2 * t8165 + 0.15476481481481481481e-2 * t5610 + t8396 * t548 + 0.74498e-1 * t4347 * t8289 - 0.23214722222222222222e-2 * t8173 + 0.15476481481481481481e-2 * t8178 - 0.23214722222222222222e-2 * t8182;
    (t8404, t8417, t8431)
}
