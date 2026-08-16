//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 965/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk965(t1882: f64, t32512: f64, t7226: f64, t8232: f64, t32517: f64, t8392: f64, t32603: f64, t32429: f64, t32465: f64, t32496: f64, t103: f64, t32325: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t137826 = t1882 * t32512;
    let t137836 = 8.0_f64 / 27.0_f64 * t8232 * t7226;
    let t137843 = t8392 * t32517;
    let t137864 = t8392 * t32603;
    let t137866 = t1882 * t32429;
    let t137872 = t1882 * t32465;
    let t137877 = t8392 * t32496;
    let t137882 = t103 * t32325;
    (t137826, t137836, t137843, t137864, t137866, t137872, t137877, t137882)
}
