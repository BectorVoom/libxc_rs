//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1004/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1004(t3088: f64, t419: f64, t85491: f64, t37749: f64, t420: f64, t85469: f64, t37389: f64, t7742: f64) -> (f64, f64, f64) {
    let t85493 = t419 * t3088 * t85491;
    let t85498 = t419 * t420 * t37749 * t85469;
    let t85501 = 24.0_f64 * t7742 + 24.0_f64 * t37389;
    (t85493, t85498, t85501)
}
