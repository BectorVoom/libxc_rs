//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1205/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1205<F: Float>(t33861: F, t33865: F, t33867: F, t33869: F, t33872: F, t36824: F, t38732: F, t38736: F, t38740: F, t38743: F, t38747: F, t38751: F, t38755: F, t38757: F, t38760: F, t38764: F, t38769: F, t38774: F) -> F {
    let t41346 = -F::cast_from(0.45017719023973223821e-1_f64) * t38732 - F::cast_from(0.12862205435420921092e-1_f64) * t38736 + F::cast_from(0.12862205435420921092e-1_f64) * t38740 + t36824 - F::new(35.0) / F::new(108.0) * t33861 + F::cast_from(0.10289764348336736873e-1_f64) * t33865 - F::cast_from(0.17149607247227894789e-2_f64) * t33867 - F::cast_from(0.20579528696673473747e-1_f64) * t38743 + F::cast_from(0.31448092289604152068e-2_f64) * t33869 - F::cast_from(0.18868855373762491241e-1_f64) * t38747 + F::cast_from(0.28303283060643736861e-1_f64) * t38751 - F::cast_from(0.12862205435420921092e-2_f64) * t38755 - F::cast_from(0.22642626448514989489e-1_f64) * t38757 - t33872 + F::cast_from(0.10289764348336736873e-1_f64) * t38760 + F::cast_from(0.10718504529517434243e-2_f64) * t38764 + F::cast_from(0.18868855373762491241e-2_f64) * t38769 - F::cast_from(0.12579236915841660828e-2_f64) * t38774;
    t41346
}
