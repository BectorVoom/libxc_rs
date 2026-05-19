//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 213/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk213<F: Float>(t626: F, t636: F, t653: F, t761: F, t762: F, t760: F, t10: F, t88: F, t131: F, t179: F, t192: F, t205: F, t571: F, t575: F, t704: F, t709: F, t713: F, t723: F, t727: F, t728: F, t739: F, t744: F, t750: F, t752: F, t754: F, t757: F, t98: F) -> (F, F, F, F, F, F, F) {
    let t766 = t761 + t762 + F::new(1.5625) * t626 + F::new(1.5625) * t636 - F::new(1.5625) * t653;
    let t767 = t760 * t766;
    let t769 = F::cast_from(0.025613155472356368_f64) * t760 + F::new(1.0);
    let t770 = F::new(1.0) / t769;
    let t772 = t770 * t88 * t10;
    let t773 = t767 * t772;
    let t776 = t571 + t575 - F::cast_from(2.2140749178833072_f64) * t704 * t98 + F::cast_from(2.2140749178833072_f64) * t192 * t709 - F::cast_from(18.635258017632964_f64) * t179 * t713 - F::cast_from(18.635258017632964_f64) * t179 * t709 + t723 + F::cast_from(2.2140749178833072_f64) * t192 * t713 - F::cast_from(0.5923479790153209_f64) * t727 * t131 * t728 + t739 + F::cast_from(2.3693919160612835_f64) * t205 * t744 + t750 - t752 - t754 - F::cast_from(22.07984838129906_f64) * t757 - F::cast_from(2.9824072957409817_f64) * t773 * t98;
    (t766, t767, t769, t770, t772, t773, t776)
}
