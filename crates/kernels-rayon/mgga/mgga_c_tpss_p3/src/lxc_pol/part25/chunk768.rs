//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 768/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk768(t450: f64, t5242: f64, t1112: f64, t242: f64, t1561: f64) -> (f64, f64, f64) {
    let t5243 = t5242 * t450;
    let t5244 = t1112 * t5243;
    let t5245 = t242 * t5244;
    let t5248 = t1561 * t1561;
    (t5243, t5245, t5248)
}
