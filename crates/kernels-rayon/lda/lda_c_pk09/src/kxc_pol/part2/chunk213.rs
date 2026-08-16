//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 213/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk213(t626: f64, t636: f64, t653: f64, t761: f64, t762: f64, t760: f64, t10: f64, t88: f64, t131: f64, t179: f64, t192: f64, t205: f64, t571: f64, t575: f64, t704: f64, t709: f64, t713: f64, t723: f64, t727: f64, t728: f64, t739: f64, t744: f64, t750: f64, t752: f64, t754: f64, t757: f64, t98: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t766 = t761 + t762 + 1.5625_f64 * t626 + 1.5625_f64 * t636 - 1.5625_f64 * t653;
    let t767 = t760 * t766;
    let t769 = 0.025613155472356368_f64 * t760 + 1.0_f64;
    let t770 = 1.0_f64 / t769;
    let t772 = t770 * t88 * t10;
    let t773 = t767 * t772;
    let t776 = t571 + t575 - 2.2140749178833072_f64 * t704 * t98 + 2.2140749178833072_f64 * t192 * t709 - 18.635258017632964_f64 * t179 * t713 - 18.635258017632964_f64 * t179 * t709 + t723 + 2.2140749178833072_f64 * t192 * t713 - 0.5923479790153209_f64 * t727 * t131 * t728 + t739 + 2.3693919160612835_f64 * t205 * t744 + t750 - t752 - t754 - 22.07984838129906_f64 * t757 - 2.9824072957409817_f64 * t773 * t98;
    (t766, t767, t769, t770, t772, t773, t776)
}
