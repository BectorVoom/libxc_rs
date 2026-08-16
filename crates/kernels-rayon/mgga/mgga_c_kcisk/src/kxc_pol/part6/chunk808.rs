//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 808/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk808(t4811: f64, t8678: f64, t5074: f64, t8951: f64, t1333: f64, t8862: f64, t1907: f64, t8964: f64, t1871: f64, t9014: f64, t4265: f64, t8999: f64, sigma2: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t23976 = t4811 * t8678;
    let t23978 = t5074 * t8951;
    let t24073 = t1333 * t8862;
    let t24081 = t8964 * t1907;
    let t24202 = t9014 * t1871;
    let t24203 = t24202 * sigma2;
    let t24299 = t4265 * t8999;
    (t23976, t23978, t24073, t24081, t24203, t24299)
}
