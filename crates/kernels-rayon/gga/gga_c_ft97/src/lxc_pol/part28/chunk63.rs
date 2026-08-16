//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 63/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk63(t25: f64, t31: f64, t120: f64) -> (f64, f64, f64, f64) {
    let t122 = t25 * t25;
    let t123 = t122 * t25;
    let t126 = f64::exp(-0.16390970575e0_f64 * t123 * t31);
    let t128 = 0.1247511874e1_f64 - 0.859614445e0_f64 * t120 + 0.812904345e0_f64 * t126;
    (t122, t123, t126, t128)
}
