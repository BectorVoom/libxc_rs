//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 798/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk798(t193: f64, t34024: f64, t33818: f64, t33845: f64, t33815: f64, t33825: f64, t33833: f64, t33838: f64, t33842: f64, t33850: f64, t33854: f64, t33857: f64, t33862: f64) -> (f64, f64, f64, f64) {
    let t34025 = t193 * t34024;
    let t34031 = 2.0_f64 / 3.0_f64 * t33818;
    let t34036 = t33845 / 3.0_f64;
    let t34041 = 3.0_f64 / 2.0_f64 * t33815 + t34031 + 2.0_f64 / 3.0_f64 * t33825 + 4.0_f64 * t33833 - 2.0_f64 * t33838 - t33842 / 2.0_f64 - t34036 - t33850 / 3.0_f64 - 3.0_f64 * t33854 + 2.0_f64 * t33857 + t33862 / 4.0_f64;
    (t34025, t34031, t34036, t34041)
}
