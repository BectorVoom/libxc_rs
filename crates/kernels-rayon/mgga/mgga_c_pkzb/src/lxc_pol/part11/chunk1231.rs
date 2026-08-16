//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1231/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1231(t30293: f64, t7375: f64, t7378: f64, t10800: f64, t17444: f64, t667: f64, t2759: f64, t9171: f64, t17351: f64, t17405: f64, t17566: f64, t20705: f64, t25633: f64, t25636: f64, t25734: f64, t25740: f64, t25747: f64, t25750: f64, t25767: f64, t30284: f64, t30287: f64, t30289: f64, t30291: f64) -> (f64, f64, f64, f64, f64) {
    let t30294 = t7375 * t30293;
    let t30296 = t7378 * t30293;
    let t30309 = t17444 * t10800 * t667;
    let t30311 = t9171 * t2759;
    let t30313 = -0.7302814814814814815e0_f64 * t17405 - 0.27903555555555555556e1_f64 * t20705 + 0.1898925e1_f64 * t30289 + 0.3071625e0_f64 * t30291 + 0.427258125e1_f64 * t30294 - 0.230371875e0_f64 * t30296 + t17566 - 0.93011851851851851854e0_f64 * t17351 + 0.11958666666666666667e1_f64 * t25633 - 0.89690000000000000001e0_f64 * t25636 + 0.82156666666666666665e0_f64 * t25734 - 0.29896666666666666667e0_f64 * t30284 + 0.8969e0_f64 * t30287 - 0.98587999999999999998e0_f64 * t25740 - 0.49293999999999999999e0_f64 * t25747 - 0.49293999999999999999e0_f64 * t25750 + 0.82156666666666666665e0_f64 * t25767 + 0.1151859375e0_f64 * t30309 - 0.230371875e0_f64 * t30311;
    (t30294, t30296, t30309, t30311, t30313)
}
