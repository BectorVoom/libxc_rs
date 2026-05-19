//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1388/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1388<F: Float>(t15988: F, t16631: F, t11882: F, t11884: F, t11914: F, t15971: F, t15976: F, t15983: F, t15986: F, t16612: F, t16615: F, t16620: F, t16625: F, t16627: F, t16629: F, t16636: F, t16640: F, t16644: F, t16651: F) -> F {
    let t17995 = F::cast_from(0.23214722222222222222e-2_f64) * t15988;
    let t18002 = F::cast_from(0.23214722222222222222e-2_f64) * t16631;
    let t18008 = -F::cast_from(0.23214722222222222222e-2_f64) * t15971 - F::new(0.10446625e-1) * t15976 - F::cast_from(0.51588271604938271604e-3_f64) * t11882 + F::cast_from(0.15476481481481481481e-2_f64) * t11884 + F::cast_from(0.10317654320987654321e-2_f64) * t15983 - F::cast_from(0.61905925925925925924e-2_f64) * t15986 - t17995 - F::cast_from(0.17411041666666666666e-2_f64) * t16612 - F::cast_from(0.38691203703703703703e-3_f64) * t16615 + F::cast_from(0.10317654320987654321e-2_f64) * t16620 + F::cast_from(0.34822083333333333332e-2_f64) * t16625 + F::cast_from(0.61905925925925925924e-2_f64) * t16627 - F::cast_from(0.41270617283950617282e-2_f64) * t16629 - t18002 - F::cast_from(0.51588271604938271604e-3_f64) * t16636 - F::cast_from(0.15476481481481481481e-2_f64) * t16640 - F::cast_from(0.15476481481481481481e-2_f64) * t16644 - F::cast_from(0.15476481481481481481e-2_f64) * t11914 + F::cast_from(0.61905925925925925924e-2_f64) * t16651;
    t18008
}
