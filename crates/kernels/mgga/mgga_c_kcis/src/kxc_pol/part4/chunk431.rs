//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 431/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk431<F: Float>(t169: F, t174: F, t1650: F, t176: F, t1649: F, t44: F, t1646: F, t234: F, t441: F, t330: F, zeta_threshold: F) -> (F, F, F, F) {
    let t170 = t169 <= zeta_threshold;
    let t175 = t174 <= zeta_threshold;
    let t1653 = piecewise3::<F>(t175, F::cast_from(0.0_f64), F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t176 * t1650);
    let t1655 = (t1649 + t1653) * t44;
    let t1657 = piecewise3::<F>(t170, F::cast_from(0.0_f64), t1646);
    let t1658 = t234 * t1657;
    let t1659 = t1658 * t441;
    let t1662 = t330 * t1646;
    (t1655, t1658, t1659, t1662)
}
