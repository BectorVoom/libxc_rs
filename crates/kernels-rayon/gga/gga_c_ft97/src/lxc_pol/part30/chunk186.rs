//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 186/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk186(t238: f64, t1408: f64, t1412: f64, t1417: f64, t1420: f64) -> f64 {
    let t239 = 0.1e-59_f64 < t238;
    let t1424 = piecewise3(t239, 2.0_f64 * t1412 - 0.22227677429409423704e-2_f64 * t238 * t1408 - 0.19153082513888888889e-1_f64 * t1417 * t1420, 0.0_f64);
    t1424
}
