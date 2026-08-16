//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 326/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk326(t194: f64, t457: f64, t201: f64, t211: f64, t214: f64, t1965: f64) -> (f64, f64, f64, f64) {
    let t1976 = t194 * t457;
    let t1977 = t1976 * t201;
    let t1978 = t211 * t214;
    let t1979 = t1965 * t1978;
    (t1976, t1977, t1978, t1979)
}
