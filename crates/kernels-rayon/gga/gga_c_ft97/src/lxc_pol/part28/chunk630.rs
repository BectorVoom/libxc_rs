//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 630/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk630(t22980: f64, t22991: f64, t23016: f64, t23029: f64, t23038: f64, t25926: f64, t25931: f64, t25935: f64, t25940: f64, t25944: f64, t25946: f64, t25948: f64) -> f64 {
    let t26089 = -t25926 / 3.0_f64 + t25931 / 9.0_f64 - t25935 / 3.0_f64 - t22980 / 3.0_f64 - t22991 / 9.0_f64 + t25940 / 3.0_f64 + t25944 / 3.0_f64 - t25946 / 9.0_f64 - t25948 / 18.0_f64 - t23016 / 12.0_f64 + t23029 / 6.0_f64 - t23038;
    t26089
}
