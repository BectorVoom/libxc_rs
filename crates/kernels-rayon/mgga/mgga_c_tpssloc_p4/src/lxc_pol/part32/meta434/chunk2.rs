//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1673/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1673(t1343: f64, t19732: f64, t820: f64, t120: f64, t6387: f64, t5248: f64, t5250: f64, t5234: f64, t5245: f64, t12283: f64, t6396: f64, t3805: f64, t3807: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19868 = t1343 * t820 * t19732;
    let t19871 = t120 * t6387;
    let t19873 = t5248 * t19871 * t5250;
    let t19876 = t5234 * t5245;
    let t19879 = t12283 * t6396;
    let t19882 = t3805 * t19871 * t3807;
    (t19868, t19871, t19873, t19876, t19879, t19882)
}
