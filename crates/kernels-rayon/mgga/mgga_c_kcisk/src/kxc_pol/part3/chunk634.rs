//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 634/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk634(t156: f64, t3122: f64, t3530: f64, t459: f64, t1422: f64, t119: f64, t179: f64, t1173: f64, t416: f64, t458: f64, t1273: f64, t4129: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5827 = t156 * t3122;
    let t5895 = t3530 * t459;
    let t5907 = t1422 * t459;
    let t5911 = t179 * t119;
    let t5926 = t416 * t1173;
    let t5953 = t416 * t458;
    let t6125 = t4129 * t1273;
    (t5827, t5895, t5907, t5911, t5926, t5953, t6125)
}
