//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 600/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk600(t1597: f64, t8403: f64, t1557: f64, t2332: f64, t4519: f64, t5880: f64, t5972: f64, t5979: f64, t6426: f64, t7834: f64, t7837: f64, t7840: f64, t7909: f64, t8075: f64, t8289: f64) -> (f64, f64) {
    let t8404 = t8403 * t1597;
    let t8417 = 0.193e0_f64 * t1557 * t8289 - 0.193e0_f64 * t1557 * t8404 - 0.386e0_f64 * t6426 * t2332 + 0.11607361111111111111e-2_f64 * t7834 - 0.34822083333333333332e-2_f64 * t7837 + 0.23214722222222222222e-2_f64 * t7840 - 0.17411041666666666666e-2_f64 * t7909 - t4519 + 0.23214722222222222222e-2_f64 * t5880 - 0.23214722222222222222e-2_f64 * t5972 + 0.15476481481481481481e-2_f64 * t5979 + 0.34822083333333333332e-2_f64 * t8075;
    (t8404, t8417)
}
