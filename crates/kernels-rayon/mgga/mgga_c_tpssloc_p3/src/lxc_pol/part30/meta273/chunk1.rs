//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1238/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1238(t6580: f64, t6587: f64, t6603: f64, t6618: f64, t7494: f64, t7498: f64, t7501: f64, t7504: f64, t7506: f64, t7508: f64) -> f64 {
    let t7510 = -t6580 - t7494 / 48.0_f64 - t6587 - 0.12111826828242117256e-2_f64 * t7498 - t6603 - 0.20186378047070195427e-3_f64 * t7501 + t7504 / 1536.0_f64 - t7506 / 1536.0_f64 - t6618 - t7508 / 384.0_f64;
    t7510
}
