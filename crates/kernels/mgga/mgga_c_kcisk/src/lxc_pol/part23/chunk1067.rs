//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1067/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1067<F: Float>(t19790: F, t19808: F, t19810: F, t1556: F, t6579: F, t19832: F, t19837: F, t13956: F, t13960: F, t13962: F, t1598: F, t19816: F, t19820: F, t19824: F, t19830: F, t19835: F) -> (F, F, F, F, F) {
    let t21420 = 0.23214722222222222222e-2 * t19790;
    let t21425 = 0.15476481481481481481e-2 * t19808;
    let t21426 = 0.15476481481481481481e-2 * t19810;
    let t21434 = t6579 * t1556;
    let t21438 = 0.23214722222222222222e-2 * t19832;
    let t21440 = 0.15476481481481481481e-2 * t19837;
    let t21441 = -0.30952962962962962962e-2 * t19816 + 0.25794135802469135802e-2 * t19820 - 0.15476481481481481481e-2 * t19824 - 0.51588271604938271604e-3 * t13956 + 0.10317654320987654321e-2 * t13960 + 0.11607361111111111111e-2 * t13962 - 0.386e0 * t21434 * t1598 - 0.38691203703703703703e-3 * t19830 - t21438 + 0.92858888888888888886e-2 * t19835 + t21440;
    (t21420, t21425, t21426, t21434, t21441)
}
