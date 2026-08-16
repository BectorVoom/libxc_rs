//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1205/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1205(t33861: f64, t33865: f64, t33867: f64, t33869: f64, t33872: f64, t36824: f64, t38732: f64, t38736: f64, t38740: f64, t38743: f64, t38747: f64, t38751: f64, t38755: f64, t38757: f64, t38760: f64, t38764: f64, t38769: f64, t38774: f64) -> f64 {
    let t41346 = -0.45017719023973223821e-1_f64 * t38732 - 0.12862205435420921092e-1_f64 * t38736 + 0.12862205435420921092e-1_f64 * t38740 + t36824 - 35.0_f64 / 108.0_f64 * t33861 + 0.10289764348336736873e-1_f64 * t33865 - 0.17149607247227894789e-2_f64 * t33867 - 0.20579528696673473747e-1_f64 * t38743 + 0.31448092289604152068e-2_f64 * t33869 - 0.18868855373762491241e-1_f64 * t38747 + 0.28303283060643736861e-1_f64 * t38751 - 0.12862205435420921092e-2_f64 * t38755 - 0.22642626448514989489e-1_f64 * t38757 - t33872 + 0.10289764348336736873e-1_f64 * t38760 + 0.10718504529517434243e-2_f64 * t38764 + 0.18868855373762491241e-2_f64 * t38769 - 0.12579236915841660828e-2_f64 * t38774;
    t41346
}
