//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 249/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk249(t759: f64, t761: f64, t764: f64, t771: f64, t785: f64, t794: f64, t797: f64, t803: f64) -> f64 {
    let t806 = -t759 - t761 * t764 / 48.0_f64 - t771 * t785 / 3072.0_f64 - t794 - t797 * t803 / 768.0_f64;
    t806
}
