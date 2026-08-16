//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1048/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1048(t103: f64, t108: f64, t19972: f64, t20162: f64, t4414: f64, t4501: f64, t4621: f64, t85406: f64, t85797: f64, t85882: f64, t85895: f64, t85903: f64, t86010: f64, t86321: f64, t86404: f64, t86411: f64, t984: f64) -> f64 {
    let t86559 = -3.0_f64 * t108 * t20162 * t4414 + 2.0_f64 * t103 * t86404 - 8.0_f64 * t19972 * t984 - 6.0_f64 * t4501 * t4621 - 48.0_f64 * t85406 + 48.0_f64 * t85797 - 72.0_f64 * t85882 + 16.0_f64 * t85895 + 24.0_f64 * t85903 - 8.0_f64 * t86010 + 48.0_f64 * t86321 - 12.0_f64 * t86411;
    t86559
}
