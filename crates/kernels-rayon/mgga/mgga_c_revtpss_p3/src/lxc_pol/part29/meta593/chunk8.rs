//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1985/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1985(t213: f64, t28888: f64, t10073: f64, t25937: f64, t7282: f64, t8085: f64, t13743: f64, t1444: f64, t1445: f64, t25921: f64, t27868: f64, t28792: f64, t28830: f64, t28911: f64, t48074: f64, t7295: f64, t7296: f64, t7511: f64, t96473: f64, t96486: f64, t96491: f64, t96500: f64, t96503: f64, t96506: f64, t96510: f64) -> f64 {
    let t102594 = t213 * t28888;
    let t102610 = t10073 * t7282 * t25937 * t8085;
    let t102612 = 0.17347256376410398924e1_f64 * t7295 * t7296 * t28888 * t1444 - 0.26020884564615598386e1_f64 * t27868 * t28911 * t48074 - t96473 - 0.13170898365871023197e1_f64 * t102594 * t1445 + 0.9757440539382783019e-2_f64 * t96486 + t96491 + 0.8673628188205199462e0_f64 * t25921 * t28792 + 0.8673628188205199462e0_f64 * t25921 * t28830 - 0.25702851531048074406e-1_f64 * t96500 - 0.48186823267806663678e-3_f64 * t96503 + 0.48186823267806663678e-3_f64 * t96506 - 0.34270468708064099208e-2_f64 * t96510 - 0.39512695097613069591e1_f64 * t7511 * t13743 - 0.24093411633903331839e-3_f64 * t102610;
    t102612
}
