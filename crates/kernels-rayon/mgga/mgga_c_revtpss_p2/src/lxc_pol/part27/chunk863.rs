//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 863/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk863(t10194: f64, t10259: f64, t10415: f64, t10416: f64, t1312: f64, t2322: f64, t2371: f64, t5523: f64, t670: f64, t2389: f64, t705: f64, t707: f64) -> (f64, f64) {
    let t10426 = 2.0_f64 * t10259 * t1312 + 6.0_f64 * t10416 * t670 + 6.0_f64 * t2322 * t2371 + 6.0_f64 * t2371 * t5523 + 6.0_f64 * t10194 + t10415;
    let t10428 = t705 * t2389;
    let t10430 = 12.0_f64 * t10428 * t707;
    (t10426, t10430)
}
