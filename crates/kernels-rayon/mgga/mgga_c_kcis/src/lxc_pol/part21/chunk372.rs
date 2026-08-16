//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 372/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk372(t2147: f64, t2154: f64, t2161: f64, t2165: f64, t228: f64, t899: f64, t232: f64, t234: f64) -> (f64, f64) {
    let t2167 = t2161 * t228 - t2165 * t899 - t2147 + t2154;
    let t2169 = t232 * t234;
    (t2167, t2169)
}
