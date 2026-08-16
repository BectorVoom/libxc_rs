//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 108/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk108(t27: f64, t29: f64, t321: f64, t265: f64, t32: f64) -> (f64, f64, f64) {
    let t328 = t321 * t29 * t27;
    let t331 = t27 * t32 * t265;
    let t332 = 0.33333333333333333333e-1_f64 * t331;
    let t333 = 5.0_f64 / 18.0_f64 * t328 - t332;
    (t331, t332, t333)
}
