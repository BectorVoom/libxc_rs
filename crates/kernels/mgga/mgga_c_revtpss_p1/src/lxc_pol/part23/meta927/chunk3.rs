//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3012/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3012<F: Float>(t11922: F, t23930: F, t4892: F, t1469: F, t18281: F, t4186: F, t5825: F, t1042: F, t1063: F, t11994: F, t16089: F, t16199: F, t23844: F, t23848: F, t3092: F, t3127: F, t3188: F, t4757: F, t4801: F, t4806: F, t55247: F, t55272: F, t55280: F, t6096: F, t67435: F, t67473: F, t67493: F, t67499: F, t67521: F, t67526: F, t78570: F, t78765: F) -> (F, F, F) {
    let t80038 = t4892 * t11922 * t23930;
    let t80045 = t18281 * t1469;
    let t80050 = t5825 * t4186;
    let t80081 = F::cast_from(0.85748036236139473944e-3_f64) * t67435 + F::cast_from(0.85748036236139473947e-3_f64) * t80038 + F::cast_from(0.19055119163586549765e-3_f64) * t55247 - F::cast_from(0.28582678745379824648e-3_f64) * t67473 - F::cast_from(0.42874018118069736972e-3_f64) * t67493 - F::cast_from(0.42874018118069736972e-3_f64) * t67499 - F::cast_from(0.42874018118069736972e-3_f64) * t67521 - F::cast_from(0.85748036236139473944e-3_f64) * t1063 * t1042 * t4801 * t80045 - F::cast_from(0.85748036236139473944e-3_f64) * t1063 * t1042 * t4801 * t80050 + F::cast_from(0.71456696863449561621e-3_f64) * t3188 * t23844 + F::cast_from(0.71456696863449561621e-3_f64) * t1063 * t1042 * t4806 * t80045 + F::cast_from(0.71456696863449561621e-3_f64) * t1063 * t1042 * t4806 * t80050 - F::cast_from(0.7145669686344956162e-3_f64) * t11994 * t23848 - F::cast_from(0.7145669686344956162e-3_f64) * t3127 * t1042 * t4806 * t78765 + F::cast_from(0.14291339372689912324e-2_f64) * t3127 * t1042 * t16199 * t78570 - F::cast_from(0.95275595817932748827e-4_f64) * t55272 + F::cast_from(0.42874018118069736972e-3_f64) * t67526 - t55280 - F::cast_from(0.17149607247227894789e-2_f64) * t16089 * t3092 * t6096 * t4757;
    (t80045, t80050, t80081)
}
