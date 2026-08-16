//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1053/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1053(t1181: f64, t34278: f64, t4353: f64, t599: f64, t1165: f64, t30209: f64, t5099: f64, t7351: f64, t30546: f64, t8657: f64, t30584: f64, t30586: f64, t30591: f64, t30592: f64, t34446: f64, t34449: f64, t34453: f64, t34455: f64, t34457: f64, t34459: f64, t34461: f64, t34463: f64, t34466: f64, t34468: f64) -> f64 {
    let t34472 = t34278 * t1181 * t599 * t4353;
    let t34476 = t30209 * t1165 * t7351 * t5099;
    let t34478 = t30546 * t8657;
    let t34480 = 0.64311027177104605458e-2_f64 * t30584 + 0.25724410870841842184e-2_f64 * t30586 + t30591 - 0.53592522647587171215e-3_f64 * t34446 + 0.95275595817932748827e-2_f64 * t30592 + 0.62896184579208304136e-3_f64 * t34449 + 0.53592522647587171215e-3_f64 * t34453 + 0.34299214494455789578e-2_f64 * t34455 + 0.17149607247227894789e-2_f64 * t34457 - 0.17149607247227894789e-2_f64 * t34459 - 0.34299214494455789578e-2_f64 * t34461 + 0.17149607247227894789e-2_f64 * t34463 - 0.10718504529517434243e-3_f64 * t34466 - 0.45017719023973223821e-2_f64 * t34468 - 0.2250885951198661191e-1_f64 * t34472 + 0.94344276868812456204e-3_f64 * t34476 + 0.56606566121287473722e-2_f64 * t34478;
    t34480
}
