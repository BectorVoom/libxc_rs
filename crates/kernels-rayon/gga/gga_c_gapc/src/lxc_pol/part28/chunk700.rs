//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 700/1429 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk700(t2801: f64, t959: f64, t2315: f64, t195: f64, t291: f64, t286: f64, t941: f64) -> (f64, f64, f64) {
    let t7177 = t959 * t2801;
    let t7178 = t7177 * t2315;
    let t7182 = t195 * t291;
    let t7191 = t941 * t286;
    (t7178, t7182, t7191)
}
