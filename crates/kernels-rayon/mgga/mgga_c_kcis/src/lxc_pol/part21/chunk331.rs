//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 331/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk331(t1664: f64, t921: f64, t261: f64, t920: f64) -> (f64, f64, f64) {
    let t1666 = -t921 - 0.17808333333333333333e-1_f64 * t1664;
    let t1668 = 0.62182e-1_f64 * t1666 * t261;
    let t1670 = -t920 / 3.0_f64 - t1664 / 3.0_f64;
    (t1666, t1668, t1670)
}
