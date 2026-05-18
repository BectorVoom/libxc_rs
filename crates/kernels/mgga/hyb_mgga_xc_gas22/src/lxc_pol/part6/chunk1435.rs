//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1435/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1435<F: Float>(t4540: F, t531: F, t1849: F, t535: F, t1572: F, t3972: F, t1145: F, t1620: F, t513: F, t4530: F, t11583: F, t11586: F, t26103: F, t26122: F, t26333: F, t2923: F, t31106: F, t31114: F, t31120: F, t31136: F, t3714: F, t3771: F, t3788: F, t9458: F, t9608: F, t9612: F, t9624: F, t9742: F) -> (F, F) {
    let t31150 = t531 * t4540;
    let t31154 = t535 * t1849;
    let t31155 = t1572 * t3972;
    let t31158 = t1620 * t513 * t1145;
    let t31163 = t531 * t4530;
    let t31176 = F::new(12.0) * t9742 * t31136 + F::new(8000.0) / F::new(9.0) * t9608 * t31114 - F::new(3200.0) / F::new(3.0) * t9612 * t31106 - F::new(180.0) * t9624 * t31150 * t2923 + F::new(5000.0) / F::new(81.0) * t31154 * t31155 * t31158 - F::new(48.0) * t26103 * t31120 - F::new(720.0) * t26122 * t31163 * t2923 + F::new(800.0) / F::new(9.0) * t26333 * t11583 - F::new(800.0) / F::new(9.0) * t3788 * t3771 * t3714 + F::new(3200.0) / F::new(3.0) * t9612 * t31114 + F::new(3200.0) / F::new(3.0) * t11586 * t9458;
    (t31155, t31176)
}
