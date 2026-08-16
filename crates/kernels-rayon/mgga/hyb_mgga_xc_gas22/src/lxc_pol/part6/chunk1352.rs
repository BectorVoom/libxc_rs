//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1352/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1352(t1347: f64, t2188: f64, t8853: f64, t2228: f64, t4143: f64, t6579: f64, t24497: f64, t8739: f64, t20703: f64, t20706: f64, t20960: f64, t24556: f64, t24559: f64, t24562: f64, t24825: f64, t24829: f64, t24832: f64, t25049: f64, t25129: f64, t271: f64, t28853: f64, t28856: f64, t28859: f64, t28962: f64, t29426: f64, t29430: f64, t29432: f64, t29434: f64, t29436: f64, t29438: f64, t29440: f64, t29442: f64, t29445: f64, t29448: f64, t828: f64, t8760: f64, t8770: f64, t8795: f64) -> (f64, f64, f64, f64) {
    let t29451 = 4.0_f64 * t2188 * t1347 * t8853;
    let t29454 = 0.96491876992155210402e2_f64 * t6579 * t4143 * t2228;
    let t29458 = 24.0_f64 * t24497 * t8739;
    let t29478 = t29426 - t29430 - t29432 - t29434 + t29436 + t29438 - t29440 - t29442 - t29445 - t29448 + t29451 + t29454 - 0.4155806185363551302e3_f64 * t25129 * t8770 - t29458 + 0.8276162067083744048e4_f64 * t24825 * t25049 * t828 + 0.14035736694323150897e2_f64 * t24832 * t8760 - 0.19751673498613801407e-1_f64 * t28962 - 0.77193501593724168323e3_f64 * t24829 * t8795 - 0.310907e-1_f64 * (t20960 - 0.10654518518518518518e0_f64 * t20703 + 0.22831111111111111111e-1_f64 * t20706 - 0.10654518518518518518e0_f64 * t24556 + 0.91324444444444444442e-1_f64 * t24559 - 0.34246666666666666666e-1_f64 * t24562 + 0.22831111111111111111e-1_f64 * t28859 - 0.34246666666666666666e-1_f64 * t28853 + 0.5137e-1_f64 * t28856) * t271;
    (t29451, t29454, t29458, t29478)
}
