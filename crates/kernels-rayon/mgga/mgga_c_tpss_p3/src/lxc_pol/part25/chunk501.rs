//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 501/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk501(t2143: f64, t764: f64, t159: f64, t238: f64, t210: f64, t227: f64, t767: f64) -> (f64, f64, f64, f64) {
    let t2144 = t2143 * t764;
    let t2146 = t159 * t238;
    let t2147 = t210 * t2146;
    let t2157 = 1.0_f64 / t767 / t227;
    (t2144, t2146, t2147, t2157)
}
