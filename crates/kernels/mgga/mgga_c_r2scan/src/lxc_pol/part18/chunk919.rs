//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 919/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk919<F: Float>(t423: F, t58: F, t597: F, t10649: F, t10648: F, t2281: F, t3428: F, t3430: F, t3308: F, t3457: F, t3429: F, t4176: F, t795: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10650 = t58 * t423;
    let t10651 = t10650 * t597;
    let t10652 = t10649 * t10651;
    let t10653 = t10648 * t10652;
    let t10655 = t2281 * t3428;
    let t10656 = t10655 * t3430;
    let t10657 = F::cast_from(0.15243824895787514157e-3_f64) * t10656;
    let t10659 = t3308 * t3457;
    let t10660 = t3429 * t10659;
    let t10662 = t4176 * t795;
    (t10650, t10651, t10652, t10653, t10655, t10657, t10659, t10660, t10662)
}
