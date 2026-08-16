//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 990/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk990(t12224: f64, t12237: f64, t12366: f64, t12381: f64, t12395: f64, t12413: f64, t12417: f64, t12561: f64, t12566: f64, t12579: f64, t12583: f64, t12594: f64) -> f64 {
    let t12731 = t12237 + t12366 - t12413 + t12417 - t12395 - t12594 - t12224 + t12381 + t12561 + t12579 + t12583 - t12566;
    t12731
}
