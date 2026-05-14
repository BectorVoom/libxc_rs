//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 330/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk330<F: Float>(t51: F, t1335: F, t1454: F, t1516: F, t1655: F, t213: F, t630: F, t555: F, t1222: F, t95: F, t476: F, t132: F, t747: F, t481: F, t129: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t52 = t51 <= zeta_threshold;
    let t1657 = t1335 + t1454 + t1516 + t1655;
    let t1658 = t213 * t1657;
    let t1662 = piecewise3(t52, 0.0, 2.0 * t51 * t630);
    let t1663 = t1662 * t555;
    let t1665 = t1222 * t95;
    let t1666 = t476 * t1665;
    let t1667 = 7.35994946043302 * t1666;
    let t1668 = t747 * t132;
    let t1669 = t481 * t1668;
    let t1670 = 1.800081713982063 * t1669;
    let t1671 = t129 * t95;
    (t1657, t1658, t1662, t1663, t1665, t1666, t1667, t1669, t1670, t1671)
}
