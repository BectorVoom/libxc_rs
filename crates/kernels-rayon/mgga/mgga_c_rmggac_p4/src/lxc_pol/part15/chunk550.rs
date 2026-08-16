//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 550/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk550(t1977: f64, t1982: f64, t7428: f64, t1987: f64, t2186: f64, t1969: f64, t7229: f64) -> (f64, f64, f64) {
    let t7430 = t1977 * t7428 * t1982;
    let t7431 = 0.19863479950205658386e-4_f64 * t7430;
    let t7438 = t2186 * t1987;
    let t7453 = t7229 * t1969;
    (t7431, t7438, t7453)
}
