//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 232/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk232(t1008: f64, t195: f64, t196: f64, t852: f64, t179: f64, t60: f64, t15: f64, t183: f64, t2: f64, t142: f64, t4: f64, t151: f64, t181: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1009 = 1.0_f64 / t1008;
    let t1010 = t195 * t1009;
    let t1011 = t852 * t196;
    let t1014 = t60 * t179;
    let t1015 = t1014 * t15;
    let t1016 = t183 * t2;
    let t1018 = t1016 * t4 * t142;
    let t1021 = t181 * t151;
    (t1009, t1010, t1011, t1014, t1015, t1016, t1018, t1021)
}
