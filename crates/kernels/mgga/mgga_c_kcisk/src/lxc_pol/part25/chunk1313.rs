//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1313/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1313<F: Float>(t112539: F, t112541: F, t112547: F, t112549: F, t112552: F, t116120: F, t116882: F, t116886: F, t116888: F, t116891: F, t116895: F, t116900: F, t116903: F, t32942: F, t34133: F, t9652: F, t9664: F) -> (F,) {
    let t116909 = 0.13888888888888888889e-1 * t32942 * t34133 + 0.8041666666666666667e-2 * t116882 * t9652 - 0.69444444444444444446e-2 * t112539 - 0.61728395061728395065e-2 * t116886 - 0.3684876543209876543e-3 * t116888 + 0.10416666666666666667e-1 * t9664 * t116891 + 0.88437037037037037034e-2 * t116895 + 0.20833333333333333334e-1 * t116120 * t9652 + 0.16581944444444444444e-2 * t116900 + 0.8041666666666666667e-2 * t116903 * t9652 + 0.11054629629629629629e-2 * t112541 - 0.23148148148148148148e-2 * t112547 + 0.46296296296296296298e-2 * t112549 + t112552;
    (t116909,)
}
