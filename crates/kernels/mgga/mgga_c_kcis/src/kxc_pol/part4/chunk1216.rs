//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1216/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1216<F: Float>(t16719: F, t16730: F, t16732: F, t16768: F, t12251: F, t16713: F, t16717: F, t16724: F, t16728: F, t16739: F, t16754: F, t16756: F, t16759: F, t16763: F, t16766: F, t16775: F, t16780: F, t16785: F, t16791: F) -> (F,) {
    let t18037 = 0.15476481481481481481e-2 * t16719;
    let t18040 = 0.15476481481481481481e-2 * t16730;
    let t18041 = 0.10317654320987654321e-2 * t16732;
    let t18049 = 0.15476481481481481481e-2 * t16768;
    let t18054 = 0.38691203703703703704e-2 * t16713 - 0.46429444444444444443e-2 * t16717 - t18037 + 0.23214722222222222222e-2 * t16724 - 0.51588271604938271603e-2 * t16728 + t18040 + t18041 - 0.25794135802469135802e-3 * t16739 + 0.10317654320987654321e-2 * t12251 - 0.34822083333333333332e-2 * t16754 - 0.25794135802469135802e-3 * t16756 - 0.61905925925925925924e-2 * t16759 - 0.30952962962962962962e-2 * t16763 + 0.11607361111111111111e-2 * t16766 + t18049 + 0.46429444444444444444e-2 * t16775 - 0.30952962962962962962e-2 * t16780 + 0.92858888888888888888e-2 * t16785 - 0.77382407407407407407e-2 * t16791;
    (t18054,)
}
