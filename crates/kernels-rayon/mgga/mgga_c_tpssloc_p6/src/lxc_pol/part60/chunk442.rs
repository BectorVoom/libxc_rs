//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 442/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk442(t1420: f64, t1423: f64, t2282: f64, t39: f64, t51: f64, t5408: f64, t5411: f64, t5416: f64, t5421: f64, t5424: f64, t56: f64, t33: f64) -> f64 {
    let t5427 = 5.0_f64 / 18.0_f64 * t39 * t5408 + 5.0_f64 / 6.0_f64 * t39 * t5411 + 88.0_f64 / 9.0_f64 * t5416 * t56 + 40.0_f64 / 9.0_f64 * t1420 * t1423 + 5.0_f64 / 18.0_f64 * t51 * t5421 - 5.0_f64 / 6.0_f64 * t51 * t5424 - t2282;
    let t5428 = t33 * t5427;
    t5428
}
