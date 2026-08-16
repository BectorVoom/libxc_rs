//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 450/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk450(t504: f64, t837: f64, t4035: f64, t529: f64, t124: f64, t235: f64, t1679: f64, t325: f64) -> (f64, f64, f64, f64) {
    let t5019 = t504 * t837;
    let t5026 = t4035 * t529;
    let t5048 = t235 * t124;
    let t5055 = t1679 * t325;
    (t5019, t5026, t5048, t5055)
}
