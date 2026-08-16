//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1083/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1083(t2201: f64, t2252: f64, t3319: f64, t3320: f64, t1234: f64, t2207: f64, t505: f64, t6159: f64, t6162: f64, t2185: f64, t5103: f64, t1543: f64, t5095: f64) -> (f64, f64, f64, f64, f64) {
    let t38123 = t2201 * t3319 * t3320 * t2252;
    let t38127 = t2207 * t3319 * t3320 * t1234;
    let t38130 = t6159 * t505 * t6162;
    let t38134 = t5103 * t3319 * t3320 * t2185;
    let t38138 = t5095 * t3319 * t3320 * t1543;
    (t38123, t38127, t38130, t38134, t38138)
}
