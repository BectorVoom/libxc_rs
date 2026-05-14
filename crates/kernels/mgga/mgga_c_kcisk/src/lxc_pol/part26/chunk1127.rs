//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1127/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1127<F: Float>(t32019: F, t32026: F, t32087: F, t32096: F, t33346: F, t33350: F, t33353: F, t33360: F, t33364: F, t33368: F, t33373: F, t33377: F, t9426: F, t9429: F, t9796: F, t9805: F) -> (F,) {
    let t33382 = 0.40208333333333333335e-2 * t9426 * t33346 + 0.11054629629629629629e-2 * t33350 - 0.16581944444444444444e-2 * t33353 - 0.34722222222222222223e-2 * t32019 * t9805 + 0.34722222222222222223e-2 * t32087 * t33360 + 0.16581944444444444444e-2 * t33364 + 0.33163888888888888888e-2 * t33368 + 0.10416666666666666667e-1 * t32096 * t9796 + 0.10416666666666666667e-1 * t33373 * t9429 + 0.40208333333333333335e-2 * t33377 * t9429 + 0.40208333333333333335e-2 * t32026 * t9796;
    (t33382,)
}
