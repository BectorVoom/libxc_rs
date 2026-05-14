//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1097/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1097<F: Float>(t31526: F, t31544: F, t32844: F, t32850: F, t35718: F, t35719: F, t35722: F, t35733: F, t35738: F, t35740: F, t35744: F, t37696: F, t37701: F, t37704: F, t40126: F, t40131: F, t40134: F, t40136: F) -> (F,) {
    let t41960 = 7.0 / 72.0 * t40126 + t32844 + 0.39624596284901231607e-1 * t31526 + t35718 - t35719 + t32850 + 0.51448821741683684367e-2 * t35722 - 0.34299214494455789578e-2 * t35733 + 0.10289764348336736873e-1 * t40131 + 0.66040993808168719345e-1 * t31544 - t37696 + 0.13719685797782315831e-1 * t35738 + 0.27439371595564631662e-1 * t40134 - 0.41159057393346947493e-1 * t40136 + 0.32012600194825403606e-1 * t35740 - 0.51448821741683684367e-2 * t35744 - t37701 + t37704;
    (t41960,)
}
