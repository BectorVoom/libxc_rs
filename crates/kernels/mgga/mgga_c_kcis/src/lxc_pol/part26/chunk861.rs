//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 861/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk861<F: Float>(t1430: F, t21106: F, t21110: F, t1437: F, t21073: F, t1330: F, t21078: F, t7164: F, t733: F, t7158: F, t743: F, t21020: F, t1451: F, t7186: F, t738: F, t104: F, t111: F, t11967: F, t120: F, t12003: F, t12005: F, t12009: F, t1404: F, t1445: F, t17137: F, t17143: F, t18431: F, t4858: F) -> (F,) {
    let t21685 = t1430 * t21106;
    let t21688 = t1430 * t21110;
    let t21691 = t1437 * t21073;
    let t21694 = t1330 * t21078;
    let t21704 = t733 * t7164;
    let t21706 = t743 * t7158;
    let t21708 = t1430 * t21020;
    let t21711 = t1437 * t21020;
    let t21714 = t1451 * t21020;
    let t21717 = t738 * t7186;
    let t21719 = -0.21078e-1 * t104 * t21685 - 0.28104e-1 * t4858 * t21688 - 0.1585e-2 * t111 * t21691 - 0.52833333333333333333e-3 * t111 * t21694 + t11967 - 0.10929333333333333333e-1 * t12003 + 0.35222222222222222222e-2 * t12005 + 0.39210208333333333333e-4 * t12009 + 0.11955719325063177623e-1 * t1404 * t18431 - 0.5179538907796306876e-4 * t1445 * t18431 + t17137 - t17143 - 0.15613333333333333333e-2 * t21704 + 0.23526125e-4 * t21706 - 0.3513e-2 * t104 * t21708 + 0.7925e-3 * t111 * t21711 + 0.50413125e-5 * t120 * t21714 + 0.26416666666666666667e-2 * t21717;
    (t21719,)
}
