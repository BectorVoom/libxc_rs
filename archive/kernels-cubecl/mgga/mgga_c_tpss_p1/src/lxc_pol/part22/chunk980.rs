//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 980/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk980<F: Float>(t8101: F, t3645: F, t725: F, t1352: F, t2332: F, t8107: F, t8118: F, t8121: F, t10497: F, t150: F, t190: F, t2109: F, t3572: F) -> (F, F, F, F, F, F, F, F) {
    let t10518 = F::cast_from(8.0_f64) * t8101;
    let t10520 = F::cast_from(2.0_f64) * t3645 * t725;
    let t10521 = t1352 * t2332;
    let t10522 = F::cast_from(4.0_f64) * t8107;
    let t10523 = F::cast_from(0.4883052614935078681e-3_f64) * t8118;
    let t10524 = F::cast_from(0.18311447306006545054e-3_f64) * t8121;
    let t10525 = t150 * t10497;
    let t10526 = t10525 * t190;
    let t10528 = F::cast_from(4.0_f64) * t3572 * t2109;
    (t10518, t10520, t10521, t10522, t10523, t10524, t10526, t10528)
}
