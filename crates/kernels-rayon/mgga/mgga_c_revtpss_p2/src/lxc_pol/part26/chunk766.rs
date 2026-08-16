//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 766/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk766(t9514: f64, t9517: f64, t9521: f64, t9553: f64, t9556: f64, t9560: f64, t9562: f64, t9565: f64, t9567: f64, t9569: f64, t9571: f64, t9574: f64) -> f64 {
    let t9852 = -t9553 + t9556 + t9560 + t9514 + t9562 - t9565 + t9567 - t9517 - t9521 + t9569 - t9571 - t9574;
    t9852
}
