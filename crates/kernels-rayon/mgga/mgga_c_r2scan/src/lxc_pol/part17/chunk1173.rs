//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1173/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1173(t1592: f64, t28000: f64, t3308: f64, t30292: f64, t6449: f64, t30296: f64, t6528: f64, t10810: f64, t574: f64, t9445: f64, t10868: f64, t6165: f64, t9380: f64) -> (f64, f64, f64, f64, f64) {
    let t43302 = t1592 * t3308 * t28000;
    let t43305 = t6449 * t3308 * t30292;
    let t43308 = t6528 * t3308 * t30296;
    let t43313 = t574 * t10810 * t9445;
    let t43316 = t6165 * t10868 * t9380;
    (t43302, t43305, t43308, t43313, t43316)
}
