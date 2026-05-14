//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 970/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk970<F: Float>(t187: F, t5586: F, t15934: F, t15988: F, t16631: F, t16719: F, t16730: F, t16732: F, t16768: F, t16793: F, t16804: F, t16806: F, t16808: F, t2096: F, t4422: F, t5713: F, t617: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t17942 = t187 * t5586;
    let t17973 = 0.15476481481481481481e-2 * t15934;
    let t17995 = 0.23214722222222222222e-2 * t15988;
    let t18002 = 0.23214722222222222222e-2 * t16631;
    let t18037 = 0.15476481481481481481e-2 * t16719;
    let t18040 = 0.15476481481481481481e-2 * t16730;
    let t18041 = 0.10317654320987654321e-2 * t16732;
    let t18049 = 0.15476481481481481481e-2 * t16768;
    let t18056 = 0.15476481481481481481e-2 * t16793;
    let t18059 = 0.23214722222222222222e-2 * t16804;
    let t18060 = 0.15476481481481481481e-2 * t16806;
    let t18061 = 0.15476481481481481481e-2 * t16808;
    let t18069 = t2096 * t4422;
    let t18079 = t5713 * t617;
    (t17942, t17973, t17995, t18002, t18037, t18040, t18041, t18049, t18056, t18059, t18060, t18061, t18069, t18079)
}
