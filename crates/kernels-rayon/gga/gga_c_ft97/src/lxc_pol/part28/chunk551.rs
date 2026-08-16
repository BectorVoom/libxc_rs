//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 551/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk551(t108: f64, t5617: f64, t1307: f64, t497: f64, t5494: f64, t92: f64, t487: f64, t5704: f64) -> (f64, f64, f64, f64) {
    let t22917 = t5617 * t108;
    let t22922 = t1307 * t497;
    let t22935 = t5494 * t92;
    let t22940 = t5704 * t487;
    (t22917, t22922, t22935, t22940)
}
