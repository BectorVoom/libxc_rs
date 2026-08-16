//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1435/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1435(t4540: f64, t531: f64, t1849: f64, t535: f64, t1572: f64, t3972: f64, t1145: f64, t1620: f64, t513: f64, t4530: f64, t11583: f64, t11586: f64, t26103: f64, t26122: f64, t26333: f64, t2923: f64, t31106: f64, t31114: f64, t31120: f64, t31136: f64, t3714: f64, t3771: f64, t3788: f64, t9458: f64, t9608: f64, t9612: f64, t9624: f64, t9742: f64) -> (f64, f64) {
    let t31150 = t531 * t4540;
    let t31154 = t535 * t1849;
    let t31155 = t1572 * t3972;
    let t31158 = t1620 * t513 * t1145;
    let t31163 = t531 * t4530;
    let t31176 = 12.0_f64 * t9742 * t31136 + 8000.0_f64 / 9.0_f64 * t9608 * t31114 - 3200.0_f64 / 3.0_f64 * t9612 * t31106 - 180.0_f64 * t9624 * t31150 * t2923 + 5000.0_f64 / 81.0_f64 * t31154 * t31155 * t31158 - 48.0_f64 * t26103 * t31120 - 720.0_f64 * t26122 * t31163 * t2923 + 800.0_f64 / 9.0_f64 * t26333 * t11583 - 800.0_f64 / 9.0_f64 * t3788 * t3771 * t3714 + 3200.0_f64 / 3.0_f64 * t9612 * t31114 + 3200.0_f64 / 3.0_f64 * t11586 * t9458;
    (t31155, t31176)
}
