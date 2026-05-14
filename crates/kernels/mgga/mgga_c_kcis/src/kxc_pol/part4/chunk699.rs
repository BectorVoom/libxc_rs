//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 699/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk699<F: Float>(t1341: F, t187: F, t1357: F, t1585: F, t3855: F, t3858: F, t3865: F, t3896: F, t3904: F, t3911: F, t3921: F, t3940: F, t3948: F, t4377: F, t4114: F, t1592: F, t3725: F, t3729: F, t3731: F, t3736: F, t3740: F, t3957: F, t4112: F, t4117: F, t4127: F, t4315: F, t626: F) -> (F, F, F, F) {
    let t4381 = t187 * t1341;
    let t4390 = -t3855 + t3858 - t3865 + t3896 + t3904 + t187 * t4377 + 0.19751789702565206229e-1 * t187 * t3911 - 0.11696446794910408142e1 * t4381 * t1357 + 0.11696446794910408142e1 * t1585 * t3921 - 0.58482233974552040708e0 * t1585 * t3940 - 0.17315755899375863299e2 * t1585 * t3948;
    let t4399 = 0.38691203703703703703e-3 * t4114;
    let t4402 = 0.66725e-1 * t1592 * t4315 + t4390 * t626 + 0.11607361111111111111e-2 * t3725 - 0.23214722222222222222e-2 * t3729 + 0.15476481481481481481e-2 * t3731 - 0.34822083333333333332e-2 * t3736 + 0.23214722222222222222e-2 * t3740 - 0.17411041666666666666e-2 * t3957 + 0.17411041666666666666e-2 * t4112 - t4399 + 0.23214722222222222222e-2 * t4117 + 0.34822083333333333332e-2 * t4127;
    (t4381, t4390, t4399, t4402)
}
