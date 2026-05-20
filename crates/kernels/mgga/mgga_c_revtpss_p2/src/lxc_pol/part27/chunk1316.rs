//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1316/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1316<F: Float>(t26866: F, t3746: F, t12773: F, t26867: F, t12904: F, t7618: F, t12777: F, t12781: F, t12789: F, t12842: F, t12847: F, t13002: F, t13008: F, t13022: F, t13029: F, t13048: F, t13055: F, t29097: F, t29100: F, t3606: F, t3631: F, t7607: F, t97204: F, t97206: F, t97211: F, t97215: F, t97218: F, t97220: F, t97222: F) -> F {
    let t97232 = t3746 * t26866;
    let t97239 = t26867 * t12773;
    let t97247 = t7618 * t12904;
    let t97249 = t97204 / F::new(216.0) + F::cast_from(0.25724410870841842183e-2_f64) * t97206 * t3606 + F::cast_from(0.25724410870841842183e-2_f64) * t97211 * t13048 - F::cast_from(0.25724410870841842183e-2_f64) * t97215 * t13055 + F::cast_from(0.17149607247227894789e-2_f64) * t97218 - t97220 / F::new(288.0) - t97222 / F::new(144.0) - t7607 * t13002 / F::new(288.0) - t7607 * t13008 / F::new(48.0) + t7607 * t13022 / F::new(36.0) - F::new(7.0) / F::new(648.0) * t7607 * t13029 - F::cast_from(0.17149607247227894789e-2_f64) * t97232 * t3631 - F::cast_from(0.17149607247227894789e-2_f64) * t29097 * t12842 + F::cast_from(0.85748036236139473944e-3_f64) * t29100 * t12847 - F::cast_from(0.11433071498151929859e-2_f64) * t97239 - F::cast_from(0.85748036236139473944e-3_f64) * t26867 * t12777 - F::cast_from(0.17149607247227894789e-2_f64) * t26867 * t12781 + F::cast_from(0.14291339372689912324e-2_f64) * t26867 * t12789 - F::cast_from(0.28582678745379824648e-3_f64) * t97247;
    t97249
}
