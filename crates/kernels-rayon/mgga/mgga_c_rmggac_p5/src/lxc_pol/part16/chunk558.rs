//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 558/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk558(t262: f64, t830: f64, t661: f64, t655: f64, t265: f64, t321: f64, t793: f64, t27: f64, t3814: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7581 = t262 * t830;
    let t7582 = t661 * t7581;
    let t7594 = t655 * t7581;
    let t7596 = t265 * t321;
    let t7597 = t793 * t7596;
    let t7599 = t3814 * t27;
    (t7581, t7582, t7594, t7596, t7597, t7599)
}
