//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 640/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk640(t1118: f64, t6020: f64, t1099: f64, t3315: f64, t5988: f64, t3313: f64, t3319: f64, t4721: f64, t5973: f64, t5977: f64, t5981: f64, t1682: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6021 = t6020 * t1118;
    let t6023 = 1.0_f64 * t1099 * t6021;
    let t6024 = t5988 * t3315;
    let t6026 = 0.16081979498692535067e2_f64 * t3313 * t6024;
    let t6031 = t3319 - 0.11415555555555555555e-1_f64 * t4721 - 0.11415555555555555555e-1_f64 * t5973 + 0.34246666666666666666e-1_f64 * t5977 + 0.17123333333333333333e-1_f64 * t5981;
    let t6036 = t1682 * t1682;
    (t6021, t6023, t6024, t6026, t6031, t6036)
}
