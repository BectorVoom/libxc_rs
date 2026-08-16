//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1191/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1191(t1058: f64, t10936: f64, t3030: f64, t990: f64, t3032: f64, t3129: f64, t3038: f64, t2775: f64, t283: f64, t3185: f64, t3199: f64, t1014: f64, t10471: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10937 = t1058 * t10936;
    let t10947 = t990 * t3030;
    let t10948 = t10947 * t3032;
    let t10949 = t10948 * t3129;
    let t10952 = t10948 * t3038;
    let t10969 = 1.0_f64 / t283 / t2775;
    let t11034 = t10947 * t3185;
    let t11037 = t10947 * t3199;
    let t11045 = t10471 * t1014;
    (t10937, t10949, t10952, t10969, t11034, t11037, t11045)
}
