//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1102/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1102<F: Float>(t35113: F, t1181: F, t23718: F, t7351: F, t7575: F, t1165: F, t4263: F, t8600: F, t30924: F, t30928: F, t30926: F, t30932: F, t30935: F, t30938: F, t30945: F, t35084: F, t35089: F, t35090: F, t35093: F, t35097: F, t35101: F, t35105: F, t35109: F) -> F {
    let t35114 = F::cast_from(0.94344276868812456204e-2_f64) * t35113;
    let t35117 = t7575 * t1181 * t7351 * t23718;
    let t35121 = t7575 * t1165 * t8600 * t4263;
    let t35123 = F::cast_from(0.75475421495049964964e-2_f64) * t30924;
    let t35125 = F::cast_from(0.75475421495049964964e-2_f64) * t30928;
    let t35130 = -F::cast_from(0.21437009059034868486e-3_f64) * t35084 + t35089 + F::cast_from(0.56606566121287473722e-2_f64) * t35090 - t35093 - t35097 - t35101 + F::cast_from(0.21437009059034868486e-3_f64) * t35105 - F::cast_from(0.10718504529517434243e-3_f64) * t35109 + t35114 - F::cast_from(0.94344276868812456204e-2_f64) * t35117 + F::cast_from(0.18868855373762491241e-1_f64) * t35121 + t35123 - F::cast_from(0.11321313224257494744e-1_f64) * t30926 - t35125 - F::cast_from(0.18868855373762491241e-1_f64) * t30932 - F::cast_from(0.11321313224257494744e-1_f64) * t30935 + F::cast_from(0.18868855373762491241e-2_f64) * t30938 + F::cast_from(0.31448092289604152068e-3_f64) * t30945;
    t35130
}
