//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1164/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1164(t139390: f64, t139410: f64, t139413: f64, t139416: f64, t148545: f64, t148551: f64, t148555: f64, t148559: f64, t148563: f64, t148567: f64, t148571: f64, t148573: f64, t148578: f64, t148580: f64, t148583: f64, t148587: f64) -> f64 {
    let t148831 = -t139390 / 18.0_f64 - 2.0_f64 / 3.0_f64 * t148545 - 2.0_f64 / 3.0_f64 * t139410 + 2.0_f64 * t139413 - 4.0_f64 / 3.0_f64 * t139416 - 4.0_f64 / 3.0_f64 * t148551 - 12.0_f64 * t148555 - t148559 / 2.0_f64 + t148563 / 3.0_f64 - 4.0_f64 / 3.0_f64 * t148567 + 3.0_f64 / 2.0_f64 * t148571 - t148573 / 9.0_f64 + t148578 / 3.0_f64 + 2.0_f64 / 9.0_f64 * t148580 - t148583 / 12.0_f64 - t148587 / 2.0_f64;
    t148831
}
