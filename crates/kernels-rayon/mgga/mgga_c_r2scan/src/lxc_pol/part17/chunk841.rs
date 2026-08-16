//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 841/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk841(t3034: f64, t5: f64, t736: f64, t5307: f64, t5321: f64, t5327: f64, t7685: f64, t7688: f64, t7689: f64, t7691: f64, t7694: f64, t7699: f64, t7701: f64, t7707: f64) -> f64 {
    let t8908 = t3034 * t5;
    let t8909 = t8908 * t736;
    let t8912 = t5307 + t5321 + 0.1350520664e0_f64 * t5327 - 0.23392894490538584828e1_f64 * t7685 + t7688 + 0.69263436422725855035e2_f64 * t7689 + 0.34631718211362927518e2_f64 * t7691 - 0.8103123984e0_f64 * t7694 + 0.2701041328e0_f64 * t7699 - 0.54217906501508699211e-2_f64 * t8909 + 24.0_f64 * t7701 - t7707;
    t8912
}
