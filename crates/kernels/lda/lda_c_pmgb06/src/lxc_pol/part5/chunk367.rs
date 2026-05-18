//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 367/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk367<F: Float>(t489: F, t530: F, t161: F, t511: F, t517: F, t187: F, t540: F, t534: F, t199: F, t718: F, t1329: F, t391: F, t566: F) -> (F, F, F, F, F, F, F, F) {
    let t1636 = t489 * t530;
    let t1637 = t161 * t1636;
    let t1639 = t511 * t517;
    let t1645 = F::new(8.0) / F::new(3.0) * t540 * t187;
    let t1646 = t534 * t187;
    let t1658 = F::new(0.1675256410710088) * t718 * t199;
    let t1659 = t1329 * t199;
    let t1661 = t391 * t566;
    (t1636, t1637, t1639, t1645, t1646, t1658, t1659, t1661)
}
