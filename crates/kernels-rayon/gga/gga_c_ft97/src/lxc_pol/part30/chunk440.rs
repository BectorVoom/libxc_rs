//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 440/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk440(t7105: f64, t840: f64, t871: f64, t1501: f64, t4246: f64, t296: f64, t1248: f64) -> (f64, f64, f64) {
    let t7107 = t840 * t871 * t7105;
    let t7110 = t4246 * t1501;
    let t7111 = t296 * t7110;
    let t7114 = t1501 * t1248;
    (t7107, t7111, t7114)
}
