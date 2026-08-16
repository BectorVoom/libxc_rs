//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 370/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk370<F: Float>(t1088: F, t1653: F, t123: F, t1087: F, t423: F, t1086: F, t1100: F, t1107: F, t1113: F, t136: F, t1105: F, t1112: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t1654 = t1088 * t1653;
    let t1655 = t123 * t1654;
    let t1657 = -t1087 + F::cast_from(0.17808333333333333333e-1_f64) * t1655;
    let t1659 = F::cast_from(0.621814e-1_f64) * t1657 * t423;
    let t1661 = -t1086 / F::cast_from(3.0_f64) + t1655 / F::cast_from(3.0_f64);
    let t1662 = t1100 * t1661;
    let t1665 = t1107 * t1661;
    let t1667 = t1113 * t1653;
    let t1668 = t136 * t1667;
    let t1670 = F::cast_from(0.1898925e1_f64) * t1662 - t1105 + F::cast_from(0.29896666666666666667e0_f64) * t1655 + F::cast_from(0.3071625e0_f64) * t1665 - t1112 + F::cast_from(0.82156666666666666667e-1_f64) * t1668;
    (t1654, t1655, t1657, t1659, t1661, t1662, t1665, t1667, t1668, t1670)
}
