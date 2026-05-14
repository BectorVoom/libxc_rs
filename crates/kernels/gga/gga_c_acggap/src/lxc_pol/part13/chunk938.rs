//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 938/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk938<F: Float>(t30584: F, t30586: F, t30591: F, t30592: F, t34446: F, t34449: F, t34453: F, t34455: F, t34457: F, t34459: F, t34461: F, t34463: F, t34466: F, t34468: F, t34472: F, t34476: F, t34478: F) -> (F,) {
    let t34480 = 0.64311027177104605458e-2 * t30584 + 0.25724410870841842184e-2 * t30586 + t30591 - 0.53592522647587171215e-3 * t34446 + 0.95275595817932748827e-2 * t30592 + 0.62896184579208304136e-3 * t34449 + 0.53592522647587171215e-3 * t34453 + 0.34299214494455789578e-2 * t34455 + 0.17149607247227894789e-2 * t34457 - 0.17149607247227894789e-2 * t34459 - 0.34299214494455789578e-2 * t34461 + 0.17149607247227894789e-2 * t34463 - 0.10718504529517434243e-3 * t34466 - 0.45017719023973223821e-2 * t34468 - 0.2250885951198661191e-1 * t34472 + 0.94344276868812456204e-3 * t34476 + 0.56606566121287473722e-2 * t34478;
    (t34480,)
}
