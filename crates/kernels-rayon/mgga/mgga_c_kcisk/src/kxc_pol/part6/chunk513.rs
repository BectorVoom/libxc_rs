//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 513/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk513(t2206: f64, t970: f64, t1333: f64, t2214: f64, t1413: f64, t2211: f64, t2218: f64, t3521: f64, t3530: f64, t459: f64, t1422: f64, t119: f64, t179: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5836 = t970 * t2206;
    let t5880 = t1333 * t2214;
    let t5885 = t2211 * t1413;
    let t5886 = t5885 * sigma0;
    let t5893 = t3521 * t2218;
    let t5895 = t3530 * t459;
    let t5907 = t1422 * t459;
    let t5911 = t179 * t119;
    (t5836, t5880, t5885, t5886, t5893, t5895, t5907, t5911)
}
