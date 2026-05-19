//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1068/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1068<F: Float>(t16730: F, t16732: F, t16768: F, t16793: F, t16804: F, t16806: F, t16808: F, t2096: F, t4422: F, t5713: F, t617: F, t12844: F, t6172: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t18040 = F::cast_from(0.15476481481481481481e-2_f64) * t16730;
    let t18041 = F::cast_from(0.10317654320987654321e-2_f64) * t16732;
    let t18049 = F::cast_from(0.15476481481481481481e-2_f64) * t16768;
    let t18056 = F::cast_from(0.15476481481481481481e-2_f64) * t16793;
    let t18059 = F::cast_from(0.23214722222222222222e-2_f64) * t16804;
    let t18060 = F::cast_from(0.15476481481481481481e-2_f64) * t16806;
    let t18061 = F::cast_from(0.15476481481481481481e-2_f64) * t16808;
    let t18069 = t2096 * t4422;
    let t18079 = t5713 * t617;
    let t18091 = t12844 * t6172;
    (t18040, t18041, t18049, t18056, t18059, t18060, t18061, t18069, t18079, t18091)
}
