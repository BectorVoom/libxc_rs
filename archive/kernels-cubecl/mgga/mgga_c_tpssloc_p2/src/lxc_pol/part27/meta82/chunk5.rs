//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 546/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk546<F: Float>(t1088: F, t1653: F, t123: F, t1087: F, t423: F, t1086: F) -> (F, F, F, F, F) {
    let t1654 = t1088 * t1653;
    let t1655 = t123 * t1654;
    let t1657 = -t1087 + F::cast_from(0.17808333333333333333e-1_f64) * t1655;
    let t1659 = F::cast_from(0.621814e-1_f64) * t1657 * t423;
    let t1661 = -t1086 / F::cast_from(3.0_f64) + t1655 / F::cast_from(3.0_f64);
    (t1654, t1655, t1657, t1659, t1661)
}
