//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1351/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1351<F: Float>(t11179: F, t116156: F, t116167: F, t116170: F, t116176: F, t116482: F, t121038: F, t121044: F, t121052: F, t121061: F, t121067: F, t121071: F, t1636: F, t32921: F, t32990: F, t33031: F, t34013: F, t34032: F, t34125: F, t34218: F, t35108: F, t35112: F, t9664: F) -> (F,) {
    let t121074 = t116156 - 0.69444444444444444447e-2 * t33031 * t11179 * t121038 * t1636 + 0.41666666666666666668e-1 * t33031 * t121044 - 0.18518518518518518519e-1 * t116482 * t34032 - 0.18518518518518518519e-1 * t116482 * t34013 + 0.34722222222222222223e-2 * t33031 * t121052 - 0.120625e-1 * t32921 * t35108 - 0.7369753086419753086e-3 * t116167 - t116170 - 0.55555555555555555558e-1 * t34125 * t34218 - 0.71481481481481481483e-2 * t116176 - 0.71481481481481481487e-2 * t121061 - 0.20833333333333333334e-1 * t32990 * t35112 - 0.2653111111111111111e-1 * t121067 + 0.20833333333333333334e-1 * t9664 * t121071;
    (t121074,)
}
