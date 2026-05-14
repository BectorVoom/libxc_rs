//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1051/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1051<F: Float>(t33861: F, t33865: F, t33867: F, t33869: F, t33872: F, t36824: F, t38732: F, t38736: F, t38740: F, t38743: F, t38747: F, t38751: F, t38755: F, t38757: F, t38760: F, t38764: F, t38769: F, t38774: F) -> (F,) {
    let t41346 = -0.45017719023973223821e-1 * t38732 - 0.12862205435420921092e-1 * t38736 + 0.12862205435420921092e-1 * t38740 + t36824 - 35.0 / 108.0 * t33861 + 0.10289764348336736873e-1 * t33865 - 0.17149607247227894789e-2 * t33867 - 0.20579528696673473747e-1 * t38743 + 0.31448092289604152068e-2 * t33869 - 0.18868855373762491241e-1 * t38747 + 0.28303283060643736861e-1 * t38751 - 0.12862205435420921092e-2 * t38755 - 0.22642626448514989489e-1 * t38757 - t33872 + 0.10289764348336736873e-1 * t38760 + 0.10718504529517434243e-2 * t38764 + 0.18868855373762491241e-2 * t38769 - 0.12579236915841660828e-2 * t38774;
    (t41346,)
}
