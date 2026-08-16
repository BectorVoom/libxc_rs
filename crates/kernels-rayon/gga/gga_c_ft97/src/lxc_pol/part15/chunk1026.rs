//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1026/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1026(t446: f64, t7793: f64, t85825: f64, t20182: f64, t925: f64, t7824: f64, t57435: f64, t73256: f64, t73259: f64, t73262: f64, t73276: f64, t73299: f64, t73301: f64, t86016: f64, t86020: f64, t86172: f64, t86175: f64, t86178: f64, t86181: f64) -> (f64, f64, f64, f64) {
    let t86188 = t446 * t7793 * t85825;
    let t86193 = t925 * t20182;
    let t86195 = t446 * t7824 * t86193;
    let t86197 = -15.0_f64 / 16.0_f64 * t86016 - 3.0_f64 / 4.0_f64 * t86020 + t86172 / 2.0_f64 - 12.0_f64 * t86175 + 8.0_f64 / 3.0_f64 * t86178 + 8.0_f64 * t86181 - 4.0_f64 / 3.0_f64 * t73256 + 8.0_f64 / 3.0_f64 * t73259 - 16.0_f64 / 9.0_f64 * t73262 + 4.0_f64 / 9.0_f64 * t73276 + 8.0_f64 / 3.0_f64 * t86188 + 16.0_f64 / 9.0_f64 * t57435 + 8.0_f64 / 3.0_f64 * t73299 + 8.0_f64 / 3.0_f64 * t73301 - 8.0_f64 * t86195;
    (t86188, t86193, t86195, t86197)
}
