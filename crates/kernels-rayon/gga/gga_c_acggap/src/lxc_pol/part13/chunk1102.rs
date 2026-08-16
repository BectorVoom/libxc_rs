//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1102/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1102(t35113: f64, t1181: f64, t23718: f64, t7351: f64, t7575: f64, t1165: f64, t4263: f64, t8600: f64, t30924: f64, t30928: f64, t30926: f64, t30932: f64, t30935: f64, t30938: f64, t30945: f64, t35084: f64, t35089: f64, t35090: f64, t35093: f64, t35097: f64, t35101: f64, t35105: f64, t35109: f64) -> f64 {
    let t35114 = 0.94344276868812456204e-2_f64 * t35113;
    let t35117 = t7575 * t1181 * t7351 * t23718;
    let t35121 = t7575 * t1165 * t8600 * t4263;
    let t35123 = 0.75475421495049964964e-2_f64 * t30924;
    let t35125 = 0.75475421495049964964e-2_f64 * t30928;
    let t35130 = -0.21437009059034868486e-3_f64 * t35084 + t35089 + 0.56606566121287473722e-2_f64 * t35090 - t35093 - t35097 - t35101 + 0.21437009059034868486e-3_f64 * t35105 - 0.10718504529517434243e-3_f64 * t35109 + t35114 - 0.94344276868812456204e-2_f64 * t35117 + 0.18868855373762491241e-1_f64 * t35121 + t35123 - 0.11321313224257494744e-1_f64 * t30926 - t35125 - 0.18868855373762491241e-1_f64 * t30932 - 0.11321313224257494744e-1_f64 * t30935 + 0.18868855373762491241e-2_f64 * t30938 + 0.31448092289604152068e-3_f64 * t30945;
    t35130
}
