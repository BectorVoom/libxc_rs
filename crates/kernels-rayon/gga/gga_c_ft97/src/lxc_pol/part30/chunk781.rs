//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 781/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk781(t33859: f64, t852: f64, t193: f64, t6308: f64, t33815: f64, t33819: f64, t33825: f64, t33833: f64, t33838: f64, t33842: f64, t33846: f64, t33850: f64, t33854: f64, t33857: f64) -> (f64, f64, f64) {
    let t33860 = t852 * t33859;
    let t33862 = t6308 * t193 * t33860;
    let t33864 = t33815 / 2.0_f64 + t33819 + 2.0_f64 / 9.0_f64 * t33825 + 4.0_f64 / 3.0_f64 * t33833 - 2.0_f64 / 3.0_f64 * t33838 - t33842 / 6.0_f64 - t33846 - t33850 / 9.0_f64 - t33854 + 2.0_f64 / 3.0_f64 * t33857 + t33862 / 12.0_f64;
    (t33860, t33862, t33864)
}
