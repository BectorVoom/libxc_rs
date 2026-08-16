//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1136/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1136(t143040: f64, t143112: f64, t28776: f64, t33822: f64, t33821: f64, t3628: f64, t3746: f64, t6307: f64, t143041: f64, t143042: f64, t28816: f64, t143274: f64, t143276: f64, t143321: f64, t143324: f64, t143327: f64, t153388: f64, t153390: f64, t153395: f64, t153399: f64, t153402: f64, t153405: f64, t153414: f64, t153418: f64) -> (f64, f64, f64, f64) {
    let t153422 = t143040 * t143112 * t33822 * t28776;
    let t153427 = t6307 * t3628 * t33821 * t33822 * t3746;
    let t153431 = t143040 * t143041 * t143042 * t28816;
    let t153432 = -t143274 - t153388 / 27.0_f64 - 4.0_f64 / 27.0_f64 * t153390 + t153395 / 18.0_f64 - t153399 / 3.0_f64 + t153402 / 9.0_f64 - t153405 / 9.0_f64 - t143276 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t143321 - 4.0_f64 / 9.0_f64 * t143324 - 2.0_f64 / 9.0_f64 * t143327 - 20.0_f64 / 3.0_f64 * t153414 + 8.0_f64 / 3.0_f64 * t153418 - 2.0_f64 * t153422 - 4.0_f64 / 9.0_f64 * t153427 + t153431;
    (t153422, t153427, t153431, t153432)
}
