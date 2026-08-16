//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 246/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk246(t1007: f64, t1054: f64, t1136: f64, t1140: f64, t1147: f64, t289: f64, t390: f64, sigma0: f64) -> (f64, f64) {
    let t1149 = t1136 * t289 - t1140 * t1147 - t1007 + t1054;
    let t1151 = 1.0_f64 / t390;
    let t1152 = sigma0 * t1151;
    (t1149, t1152)
}
