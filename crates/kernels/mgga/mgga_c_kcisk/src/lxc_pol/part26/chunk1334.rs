//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1334/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1334<F: Float>(t113941: F, t113947: F, t114664: F, t1163: F, t119097: F, t119351: F, t119354: F, t119357: F, t119364: F, t119366: F, t119372: F, t13472: F, t32008: F, t32019: F, t32087: F, t32096: F, t33360: F, t33373: F, t33410: F, t33424: F, t33434: F, t34799: F, t9449: F) -> (F,) {
    let t119379 = -0.34722222222222222223e-2 * t32096 * t34799 - 0.34722222222222222223e-2 * t32019 * t34799 - 0.34722222222222222223e-2 * t119097 * t9449 - 0.33163888888888888888e-2 * t119351 + 0.22109259259259259259e-2 * t119354 + 0.66327777777777777776e-2 * t119357 - 0.41666666666666666668e-1 * t33373 * t33434 + 0.69444444444444444447e-2 * t114664 * t33424 - 0.44218518518518518516e-2 * t119364 - 0.69444444444444444447e-2 * t32087 * t13472 * t119366 * t1163 + 0.26805555555555555556e-2 * t32008 * t119372 + 0.13888888888888888889e-1 * t113941 * t33410 + 0.26805555555555555556e-2 * t113947 * t33360;
    (t119379,)
}
