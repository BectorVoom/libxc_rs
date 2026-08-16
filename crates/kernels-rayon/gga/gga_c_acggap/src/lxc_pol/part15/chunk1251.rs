//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1251/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1251(t31526: f64, t31544: f64, t32844: f64, t32850: f64, t35718: f64, t35719: f64, t35722: f64, t35733: f64, t35738: f64, t35740: f64, t35744: f64, t37696: f64, t37701: f64, t37704: f64, t40126: f64, t40131: f64, t40134: f64, t40136: f64) -> f64 {
    let t41960 = 7.0_f64 / 72.0_f64 * t40126 + t32844 + 0.39624596284901231607e-1_f64 * t31526 + t35718 - t35719 + t32850 + 0.51448821741683684367e-2_f64 * t35722 - 0.34299214494455789578e-2_f64 * t35733 + 0.10289764348336736873e-1_f64 * t40131 + 0.66040993808168719345e-1_f64 * t31544 - t37696 + 0.13719685797782315831e-1_f64 * t35738 + 0.27439371595564631662e-1_f64 * t40134 - 0.41159057393346947493e-1_f64 * t40136 + 0.32012600194825403606e-1_f64 * t35740 - 0.51448821741683684367e-2_f64 * t35744 - t37701 + t37704;
    t41960
}
