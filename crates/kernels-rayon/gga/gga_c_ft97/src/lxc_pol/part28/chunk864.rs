//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 864/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk864(t34512: f64, t83: f64, t34737: f64, t34742: f64, t34746: f64, t34750: f64, t34754: f64, t34758: f64, t34762: f64, t34765: f64, t34770: f64, t34773: f64, t34776: f64, t446: f64) -> (f64, f64) {
    let t34779 = t83 * t34512;
    let t34782 = 2.0_f64 / 3.0_f64 * t446 * t34737 - 2.0_f64 / 3.0_f64 * t446 * t34742 + 4.0_f64 / 3.0_f64 * t446 * t34746 - 2.0_f64 * t446 * t34750 - t446 * t34754 / 3.0_f64 - t446 * t34758 / 3.0_f64 - t446 * t34762 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t34765 + t446 * t34770 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t34773 - 2.0_f64 / 3.0_f64 * t446 * t34776 - t446 * t34779 / 3.0_f64;
    (t34779, t34782)
}
