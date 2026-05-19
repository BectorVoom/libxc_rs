//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1311/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1311<F: Float>(t1437: F, t21020: F, t1451: F, t7186: F, t738: F, t104: F, t111: F, t11967: F, t120: F, t12003: F, t12005: F, t12009: F, t1404: F, t1445: F, t17137: F, t17143: F, t18431: F, t21685: F, t21688: F, t21691: F, t21694: F, t21704: F, t21706: F, t21708: F, t4858: F) -> F {
    let t21711 = t1437 * t21020;
    let t21714 = t1451 * t21020;
    let t21717 = t738 * t7186;
    let t21719 = -F::new(0.21078e-1) * t104 * t21685 - F::new(0.28104e-1) * t4858 * t21688 - F::new(0.1585e-2) * t111 * t21691 - F::cast_from(0.52833333333333333333e-3_f64) * t111 * t21694 + t11967 - F::cast_from(0.10929333333333333333e-1_f64) * t12003 + F::cast_from(0.35222222222222222222e-2_f64) * t12005 + F::cast_from(0.39210208333333333333e-4_f64) * t12009 + F::cast_from(0.11955719325063177623e-1_f64) * t1404 * t18431 - F::cast_from(0.5179538907796306876e-4_f64) * t1445 * t18431 + t17137 - t17143 - F::cast_from(0.15613333333333333333e-2_f64) * t21704 + F::new(0.23526125e-4) * t21706 - F::new(0.3513e-2) * t104 * t21708 + F::new(0.7925e-3) * t111 * t21711 + F::new(0.50413125e-5) * t120 * t21714 + F::cast_from(0.26416666666666666667e-2_f64) * t21717;
    t21719
}
