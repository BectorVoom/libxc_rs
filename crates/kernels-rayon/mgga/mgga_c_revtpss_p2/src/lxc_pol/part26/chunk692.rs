//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 692/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk692(t7031: f64, t7034: f64, t7041: f64, t7026: f64, t7039: f64, t7046: f64, t7391: f64) -> f64 {
    let t7393 = 0.28582678745379824648e-4_f64 * t7031;
    let t7394 = 0.50820002809285328225e-4_f64 * t7034;
    let t7396 = 0.40015750243531754507e-2_f64 * t7041;
    let t7398 = -t7391 - t7026 / 24.0_f64 - t7393 + t7394 - 0.85748036236139473944e-3_f64 * t7039 - t7396 - 0.34299214494455789578e-2_f64 * t7046;
    t7398
}
