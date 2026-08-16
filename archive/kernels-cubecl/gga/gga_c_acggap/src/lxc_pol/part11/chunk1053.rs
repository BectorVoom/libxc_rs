//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1053/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1053<F: Float>(t1181: F, t34278: F, t4353: F, t599: F, t1165: F, t30209: F, t5099: F, t7351: F, t30546: F, t8657: F, t30584: F, t30586: F, t30591: F, t30592: F, t34446: F, t34449: F, t34453: F, t34455: F, t34457: F, t34459: F, t34461: F, t34463: F, t34466: F, t34468: F) -> F {
    let t34472 = t34278 * t1181 * t599 * t4353;
    let t34476 = t30209 * t1165 * t7351 * t5099;
    let t34478 = t30546 * t8657;
    let t34480 = F::cast_from(0.64311027177104605458e-2_f64) * t30584 + F::cast_from(0.25724410870841842184e-2_f64) * t30586 + t30591 - F::cast_from(0.53592522647587171215e-3_f64) * t34446 + F::cast_from(0.95275595817932748827e-2_f64) * t30592 + F::cast_from(0.62896184579208304136e-3_f64) * t34449 + F::cast_from(0.53592522647587171215e-3_f64) * t34453 + F::cast_from(0.34299214494455789578e-2_f64) * t34455 + F::cast_from(0.17149607247227894789e-2_f64) * t34457 - F::cast_from(0.17149607247227894789e-2_f64) * t34459 - F::cast_from(0.34299214494455789578e-2_f64) * t34461 + F::cast_from(0.17149607247227894789e-2_f64) * t34463 - F::cast_from(0.10718504529517434243e-3_f64) * t34466 - F::cast_from(0.45017719023973223821e-2_f64) * t34468 - F::cast_from(0.2250885951198661191e-1_f64) * t34472 + F::cast_from(0.94344276868812456204e-3_f64) * t34476 + F::cast_from(0.56606566121287473722e-2_f64) * t34478;
    t34480
}
