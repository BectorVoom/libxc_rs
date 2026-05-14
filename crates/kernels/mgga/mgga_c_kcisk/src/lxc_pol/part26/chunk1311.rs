//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1311/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1311<F: Float>(t2173: F, t32069: F, t6204: F, t6211: F, t109883: F, t26516: F, t3482: F, t114664: F, t118812: F, t118822: F, t118827: F, t118837: F, t118840: F, t118843: F, t118846: F, t118849: F, t32013: F, t33384: F, t33389: F, t33446: F, t87991: F, t9426: F, t9446: F) -> (F, F, F) {
    let t118853 = t6204 * t32069 * t2173 * t6211;
    let t118859 = t3482 * t109883 * t26516;
    let t118861 = -0.40208333333333333335e-2 * t9426 * t118812 + 0.120625e-1 * t9426 * t118822 - 0.10416666666666666667e-1 * t9446 * t118827 - 0.41666666666666666668e-1 * t33384 * t33389 - 0.41666666666666666668e-1 * t9446 * t6204 * t32013 * t87991 + 0.33163888888888888888e-2 * t118837 - 0.22109259259259259259e-2 * t118840 + 0.99491666666666666664e-2 * t118843 - 0.16581944444444444444e-1 * t118846 - 0.13265555555555555555e-1 * t118849 - 0.8041666666666666667e-2 * t9426 * t118853 + 0.69444444444444444446e-2 * t114664 * t33446 - 0.22109259259259259259e-2 * t118859;
    (t118853, t118859, t118861)
}
