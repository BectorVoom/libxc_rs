//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 841/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk841(t1799: f64, t28287: f64, t6965: f64, t8946: f64, t1873: f64, t1869: f64, t2527: f64, t8786: f64, t1899: f64, t23033: f64, t2364: f64, t1800: f64) -> (f64, f64, f64, f64, f64) {
    let t28288 = t1799 * t28287;
    let t28290 = t6965 * t8946;
    let t28291 = t1873 * t28290;
    let t28292 = t1869 * t28291;
    let t28294 = t8786 * t2527;
    let t28295 = t1899 * t28294;
    let t28296 = t1873 * t28295;
    let t28297 = t1869 * t28296;
    let t28299 = t23033 * t2364;
    let t28300 = t1800 * t28299;
    (t28288, t28292, t28294, t28297, t28300)
}
