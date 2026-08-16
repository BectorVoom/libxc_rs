//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 994/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk994(t11402: f64, t2437: f64, t13261: f64, t1572: f64, t4673: f64, t11485: f64, t3377: f64, t10216: f64, t13253: f64, t13375: f64, t13445: f64, t1456: f64, t1457: f64, t1645: f64, t1646: f64, t2859: f64, t44329: f64, t44382: f64, t46724: f64, t46730: f64, t46732: f64, t46735: f64, t46740: f64, t46742: f64, t46754: f64, t46758: f64, t46760: f64, t46765: f64, t528: f64, t8155: f64, t8158: f64) -> f64 {
    let t46767 = 0.35750489951850426669e0_f64 * t2437 * t11402;
    let t46773 = 0.47667319935800568892e0_f64 * t1572 * t4673 * t13261;
    let t46775 = 0.25025342966295298669e1_f64 * t11485 * t3377;
    let t46776 = -t46724 + 0.95334639871601137787e0_f64 * t1572 * t4673 * t13253 - t46730 + t46732 + t46735 + 0.71500979903700853338e0_f64 * t1572 * t1457 * t44382 + t46740 + t46742 - 0.21450293971110256002e1_f64 * t8158 * t13375 - 0.21450293971110256002e1_f64 * t2859 * t1645 * t10216 - 0.35750489951850426669e0_f64 * t528 * t13445 * t1646 - t46754 - t46758 - t46760 - 0.21450293971110256002e1_f64 * t8155 * t13375 + t46765 + t46767 + 0.35750489951850426669e0_f64 * t1456 * t1457 * t44329 + t46773 - t46775;
    t46776
}
