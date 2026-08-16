//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 194/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk194<F: Float>(t38: F, t606: F, t95: F, t103: F, t100: F, t92: F, t96: F, tau0: F) -> (F, F, F, F, F) {
    let t657 = tau0 * t38;
    let t659 = t606 / F::cast_from(2.0_f64);
    let t660 = t95 * t659;
    let t662 = -t659;
    let t663 = t103 * t662;
    let t666 = F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t100 * t663 - F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t657 * t96 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t92 * t660;
    (t657, t659, t662, t663, t666)
}
