//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1281/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1281<F: Float>(t116186: F, t33033: F, t6763: F, t16631: F, t33017: F, t5054: F, t62249: F, t9664: F, t9939: F, t112209: F, t112212: F, t116167: F, t116170: F, t116174: F, t116176: F, t116181: F, t116184: F, t32893: F, t32917: F, t33031: F, t34122: F, t34154: F) -> (F, F, F) {
    let t116188 = t116186 * t6763 * t33033;
    let t116194 = t5054 * t33017 * t16631;
    let t116197 = t9664 * t62249 * t9939;
    let t116199 = -0.3684876543209876543e-3 * t116167 - t116170 + 0.40208333333333333335e-2 * t34154 * t32893 - 0.33163888888888888888e-2 * t116174 - 0.71481481481481481485e-2 * t116176 + 0.69444444444444444446e-2 * t34122 * t32917 + 0.18424382716049382715e-2 * t116181 + 0.66327777777777777776e-2 * t116184 - 0.13888888888888888889e-1 * t33031 * t116188 - 0.15432098765432098765e-2 * t112209 + 0.15432098765432098766e-2 * t112212 - 0.27636574074074074073e-2 * t116194 - 0.23148148148148148149e-2 * t116197;
    (t116188, t116194, t116199)
}
