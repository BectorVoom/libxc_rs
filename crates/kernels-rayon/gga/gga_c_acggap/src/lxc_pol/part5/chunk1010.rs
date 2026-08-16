//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1010/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1010(t17039: f64, t4737: f64, t3379: f64, t4963: f64, t14230: f64, t406: f64, t1165: f64, t1532: f64, t3456: f64, t1181: f64, t3451: f64, t360: f64, t4183: f64) -> (f64, f64, f64, f64, f64) {
    let t17040 = t17039 * t4737;
    let t17042 = t3379 * t4963;
    let t17056 = t14230 * t406;
    let t17059 = t3456 * t1165 * t1532 * t17056;
    let t17064 = t3451 * t1181 * t1532 * t4183 * t360;
    (t17040, t17042, t17056, t17059, t17064)
}
