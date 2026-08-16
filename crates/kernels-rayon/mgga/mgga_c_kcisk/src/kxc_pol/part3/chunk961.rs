//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 961/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk961(t1411: f64, t14210: f64, t1284: f64, t1299: f64, t3777: f64, t1340: f64, t3504: f64, t3739: f64, t3509: f64, t10471: f64, t140: f64, t416: f64) -> (f64, f64, f64, f64, f64) {
    let t14211 = t1411 * t14210;
    let t14213 = t1299 * t1284;
    let t14214 = t14213 * t3777;
    let t14215 = t1340 * t14214;
    let t14216 = t1411 * t14215;
    let t14218 = t3739 * t3504;
    let t14220 = t3739 * t3509;
    let t14223 = t140 * t10471 * t416;
    (t14211, t14216, t14218, t14220, t14223)
}
