//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 870/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk870<F: Float>(t7531: F, t7532: F, t7535: F, t1099: F, t2807: F, t479: F, t1101: F, t1056: F, t2775: F, t458: F, t2741: F, t2749: F, t1085: F, t7523: F, t7501: F, t7504: F, t7507: F, t7510: F, t7514: F, t7516: F, t7518: F, t7521: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t7536 = t7531 * t7532 * t7535;
    let t7538 = 0.10254018858216406658e4 * t1099 * t7536;
    let t7540 = t2807 * t479;
    let t7541 = t7540 * t1101;
    let t7543 = t1056 * t2775;
    let t7544 = t458 * t7543;
    let t7546 = t2741 * t2749;
    let t7559 = t7532 * t1085;
    let t7562 = t7523 * t1085;
    let t7565 = t479 * t7531;
    let t7566 = t7532 * t7535;
    let t7577 = -0.25319e1 * t7501 + 0.16879333333333333333e1 * t7504 - 0.19692555555555555555e1 * t7507 - 0.93011851851851851854e0 * t7510 + 0.13651666666666666667e0 * t7514 - 0.27303333333333333333e0 * t7516 - 0.3185388888888888889e0 * t7518 - 0.36514074074074074075e0 * t7521;
    (t7536, t7538, t7540, t7541, t7543, t7544, t7546, t7559, t7562, t7565, t7566, t7577)
}
