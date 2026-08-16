//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 636/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk636(t5015: f64, t8797: f64, t5020: f64, t7715: f64, t1775: f64, t1776: f64, t7718: f64, t5007: f64, t5006: f64, t2464: f64, t5031: f64, t1310: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8798 = t5015 * t8797;
    let t8801 = t5020 * t7715;
    let t8802 = t1775 * t8801;
    let t8806 = t1776 * t7718;
    let t8807 = t1775 * t8806;
    let t8810 = t5007 * t7715;
    let t8811 = t5006 * t8810;
    let t8814 = t2464 * t2464;
    let t8815 = t5031 * t8814;
    let t8816 = t1310 * t8815;
    (t8798, t8801, t8802, t8806, t8807, t8810, t8811, t8814, t8815, t8816)
}
