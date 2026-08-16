//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 994/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk994<F: Float>(t11402: F, t2437: F, t13261: F, t1572: F, t4673: F, t11485: F, t3377: F, t10216: F, t13253: F, t13375: F, t13445: F, t1456: F, t1457: F, t1645: F, t1646: F, t2859: F, t44329: F, t44382: F, t46724: F, t46730: F, t46732: F, t46735: F, t46740: F, t46742: F, t46754: F, t46758: F, t46760: F, t46765: F, t528: F, t8155: F, t8158: F) -> F {
    let t46767 = F::cast_from(0.35750489951850426669e0_f64) * t2437 * t11402;
    let t46773 = F::cast_from(0.47667319935800568892e0_f64) * t1572 * t4673 * t13261;
    let t46775 = F::cast_from(0.25025342966295298669e1_f64) * t11485 * t3377;
    let t46776 = -t46724 + F::cast_from(0.95334639871601137787e0_f64) * t1572 * t4673 * t13253 - t46730 + t46732 + t46735 + F::cast_from(0.71500979903700853338e0_f64) * t1572 * t1457 * t44382 + t46740 + t46742 - F::cast_from(0.21450293971110256002e1_f64) * t8158 * t13375 - F::cast_from(0.21450293971110256002e1_f64) * t2859 * t1645 * t10216 - F::cast_from(0.35750489951850426669e0_f64) * t528 * t13445 * t1646 - t46754 - t46758 - t46760 - F::cast_from(0.21450293971110256002e1_f64) * t8155 * t13375 + t46765 + t46767 + F::cast_from(0.35750489951850426669e0_f64) * t1456 * t1457 * t44329 + t46773 - t46775;
    t46776
}
