//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1408/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1408<F: Float>(t117409: F, t7312: F, t739: F, t24202: F, t654: F, t9705: F, t24272: F, t33121: F, t24483: F, t9708: F, t33094: F, t35316: F, t34329: F, t7307: F, t4817: F, t9069: F) -> (F, F, F, F, F, F, F) {
    let t122318 = t739 * t117409 * t7312;
    let t122320 = t24202 * t654;
    let t122321 = t122320 * t9705;
    let t122323 = t33121 * t24272;
    let t122325 = t9708 * t24483;
    let t122327 = t33094 * t35316;
    let t122329 = t34329 * t7307;
    let t122331 = t4817 * t9069;
    (t122318, t122321, t122323, t122325, t122327, t122329, t122331)
}
