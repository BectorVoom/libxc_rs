//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1282/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1282(t5484: f64, t8184: f64, t29903: f64, t30048: f64, t30279: f64, t30291: f64, t30301: f64, t30507: f64, t30510: f64, t30514: f64, t30517: f64, t30521: f64, t30524: f64, t30527: f64, t64: f64, t8128: f64, t8137: f64) -> (f64, f64) {
    let t30530 = t8184 * t5484;
    let t30533 = -t30048 - 4.0_f64 / 3.0_f64 * t30279 - 10.0_f64 / 9.0_f64 * t30291 + 10.0_f64 / 9.0_f64 * t30301 - 3.0_f64 / 4.0_f64 * t29903 * t30507 - 5.0_f64 / 6.0_f64 * t8128 * t30510 + 5.0_f64 / 6.0_f64 * t8128 * t30514 + t8128 * t30517 / 4.0_f64 - 5.0_f64 / 9.0_f64 * t64 * t30521 + 25.0_f64 / 36.0_f64 * t8137 * t30524 - 5.0_f64 / 36.0_f64 * t8137 * t30527 - 5.0_f64 / 24.0_f64 * t8137 * t30530;
    (t30530, t30533)
}
