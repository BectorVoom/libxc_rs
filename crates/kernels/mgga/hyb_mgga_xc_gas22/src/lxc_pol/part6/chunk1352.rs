//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1352/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1352<F: Float>(t1347: F, t2188: F, t8853: F, t2228: F, t4143: F, t6579: F, t24497: F, t8739: F, t20703: F, t20706: F, t20960: F, t24556: F, t24559: F, t24562: F, t24825: F, t24829: F, t24832: F, t25049: F, t25129: F, t271: F, t28853: F, t28856: F, t28859: F, t28962: F, t29426: F, t29430: F, t29432: F, t29434: F, t29436: F, t29438: F, t29440: F, t29442: F, t29445: F, t29448: F, t828: F, t8760: F, t8770: F, t8795: F) -> (F, F, F, F) {
    let t29451 = F::new(4.0) * t2188 * t1347 * t8853;
    let t29454 = F::cast_from(0.96491876992155210402e2_f64) * t6579 * t4143 * t2228;
    let t29458 = F::new(24.0) * t24497 * t8739;
    let t29478 = t29426 - t29430 - t29432 - t29434 + t29436 + t29438 - t29440 - t29442 - t29445 - t29448 + t29451 + t29454 - F::cast_from(0.4155806185363551302e3_f64) * t25129 * t8770 - t29458 + F::cast_from(0.8276162067083744048e4_f64) * t24825 * t25049 * t828 + F::cast_from(0.14035736694323150897e2_f64) * t24832 * t8760 - F::cast_from(0.19751673498613801407e-1_f64) * t28962 - F::cast_from(0.77193501593724168323e3_f64) * t24829 * t8795 - F::new(0.310907e-1) * (t20960 - F::cast_from(0.10654518518518518518e0_f64) * t20703 + F::cast_from(0.22831111111111111111e-1_f64) * t20706 - F::cast_from(0.10654518518518518518e0_f64) * t24556 + F::cast_from(0.91324444444444444442e-1_f64) * t24559 - F::cast_from(0.34246666666666666666e-1_f64) * t24562 + F::cast_from(0.22831111111111111111e-1_f64) * t28859 - F::cast_from(0.34246666666666666666e-1_f64) * t28853 + F::new(0.5137e-1) * t28856) * t271;
    (t29451, t29454, t29458, t29478)
}
