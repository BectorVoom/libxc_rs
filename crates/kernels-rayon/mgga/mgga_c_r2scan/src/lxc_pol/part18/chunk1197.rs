//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1197/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1197(t2184: f64, t30213: f64, t3308: f64, t12547: f64, t6425: f64, t1592: f64, t27996: f64, t28000: f64, t30292: f64, t6449: f64, t30296: f64, t6528: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t43294 = t2184 * t3308 * t30213;
    let t43296 = t6425 * t12547;
    let t43299 = t1592 * t3308 * t27996;
    let t43302 = t1592 * t3308 * t28000;
    let t43305 = t6449 * t3308 * t30292;
    let t43308 = t6528 * t3308 * t30296;
    (t43294, t43296, t43299, t43302, t43305, t43308)
}
