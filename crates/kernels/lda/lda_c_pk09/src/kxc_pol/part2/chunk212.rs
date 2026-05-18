//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 212/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk212<F: Float>(t170: F, t748: F, t151: F, t573: F, t161: F, t650: F, t95: F, t120: F, t119: F, t174: F, t612: F, t616: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t750 = F::new(0.027433775686566395) * t748 * t170;
    let t752 = F::new(1.2536914064583544) * t151 * t573;
    let t754 = F::new(3.2915558116322368) * t161 * t573;
    let t755 = t95 * t650;
    let t756 = t120 * t755;
    let t757 = t119 * t756;
    let t759 = t174 * t174;
    let t760 = F::new(1.0) / t759;
    let t761 = F::new(1.5625) * t612;
    let t762 = F::new(1.0416666666666667) * t616;
    (t750, t752, t754, t755, t756, t757, t759, t760, t761, t762)
}
