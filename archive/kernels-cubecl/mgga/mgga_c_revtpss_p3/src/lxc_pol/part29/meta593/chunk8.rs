//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1985/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1985<F: Float>(t213: F, t28888: F, t10073: F, t25937: F, t7282: F, t8085: F, t13743: F, t1444: F, t1445: F, t25921: F, t27868: F, t28792: F, t28830: F, t28911: F, t48074: F, t7295: F, t7296: F, t7511: F, t96473: F, t96486: F, t96491: F, t96500: F, t96503: F, t96506: F, t96510: F) -> F {
    let t102594 = t213 * t28888;
    let t102610 = t10073 * t7282 * t25937 * t8085;
    let t102612 = F::cast_from(0.17347256376410398924e1_f64) * t7295 * t7296 * t28888 * t1444 - F::cast_from(0.26020884564615598386e1_f64) * t27868 * t28911 * t48074 - t96473 - F::cast_from(0.13170898365871023197e1_f64) * t102594 * t1445 + F::cast_from(0.9757440539382783019e-2_f64) * t96486 + t96491 + F::cast_from(0.8673628188205199462e0_f64) * t25921 * t28792 + F::cast_from(0.8673628188205199462e0_f64) * t25921 * t28830 - F::cast_from(0.25702851531048074406e-1_f64) * t96500 - F::cast_from(0.48186823267806663678e-3_f64) * t96503 + F::cast_from(0.48186823267806663678e-3_f64) * t96506 - F::cast_from(0.34270468708064099208e-2_f64) * t96510 - F::cast_from(0.39512695097613069591e1_f64) * t7511 * t13743 - F::cast_from(0.24093411633903331839e-3_f64) * t102610;
    t102612
}
