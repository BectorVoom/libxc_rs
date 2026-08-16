//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 30/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk30(t93: f64, t96: f64, t62: f64, t80: f64, t85: f64, t88: f64) -> (f64, f64) {
    let t97 = t93 * t96;
    let t99 = -0.59778596625315888114e-2_f64 * t62 + 0.1317375e-2_f64 * t80 - 0.23775e-3_f64 * t85 + 0.64744236347453835951e-5_f64 * t88 - 0.540140625e-6_f64 * t97;
    (t97, t99)
}
