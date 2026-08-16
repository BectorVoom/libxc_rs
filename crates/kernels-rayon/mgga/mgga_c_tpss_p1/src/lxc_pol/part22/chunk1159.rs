//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1159/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1159(t1206: f64, t12947: f64, t3234: f64, t4452: f64, t1228: f64, t12810: f64, t1226: f64, t1229: f64, t12928: f64, t12938: f64, t12944: f64, t1634: f64, t1636: f64, t3315: f64, t3320: f64, t3323: f64, t4445: f64, t4451: f64, t4453: f64, t4456: f64, t516: f64, t518: f64) -> f64 {
    let t12948 = t12947 * t1206;
    let t12951 = t4452 * t3234;
    let t12954 = t1228 * t12810;
    let t12957 = 6.0_f64 * t1226 * t4456 + 6.0_f64 * t1229 * t4445 - t12928 * t518 - 24.0_f64 * t12938 * t4453 + 60.0_f64 * t12944 * t4451 - 24.0_f64 * t12948 * t4451 - 12.0_f64 * t12951 * t4451 + 3.0_f64 * t12954 * t516 - 12.0_f64 * t1634 * t3320 + 3.0_f64 * t1634 * t3323 + 3.0_f64 * t1636 * t3315;
    t12957
}
