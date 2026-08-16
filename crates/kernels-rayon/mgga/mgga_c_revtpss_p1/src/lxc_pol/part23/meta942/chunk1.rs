//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3094/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3094(t12361: f64, t24212: f64, t3384: f64, t5105: f64, t6470: f64, t24765: f64, t3531: f64, t1196: f64, t16988: f64, t20472: f64, t1733: f64, t20447: f64) -> (f64, f64, f64, f64, f64) {
    let t81601 = 6.0_f64 * t12361 * t24212;
    let t81604 = 6.0_f64 * t3384 * t5105 * t6470;
    let t81606 = 0.10254018858216406658e4_f64 * t3531 * t24765;
    let t81609 = 0.31168546390226634765e3_f64 * t1196 * t20472 * t16988;
    let t81612 = 6.0_f64 * t3384 * t1733 * t20447;
    (t81601, t81604, t81606, t81609, t81612)
}
