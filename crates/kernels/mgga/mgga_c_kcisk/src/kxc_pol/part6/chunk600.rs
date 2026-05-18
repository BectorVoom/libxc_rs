//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 600/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk600<F: Float>(t1597: F, t8403: F, t1557: F, t2332: F, t4519: F, t5880: F, t5972: F, t5979: F, t6426: F, t7834: F, t7837: F, t7840: F, t7909: F, t8075: F, t8289: F) -> (F, F) {
    let t8404 = t8403 * t1597;
    let t8417 = F::new(0.193e0) * t1557 * t8289 - F::new(0.193e0) * t1557 * t8404 - F::new(0.386e0) * t6426 * t2332 + F::new(0.11607361111111111111e-2) * t7834 - F::new(0.34822083333333333332e-2) * t7837 + F::new(0.23214722222222222222e-2) * t7840 - F::new(0.17411041666666666666e-2) * t7909 - t4519 + F::new(0.23214722222222222222e-2) * t5880 - F::new(0.23214722222222222222e-2) * t5972 + F::new(0.15476481481481481481e-2) * t5979 + F::new(0.34822083333333333332e-2) * t8075;
    (t8404, t8417)
}
