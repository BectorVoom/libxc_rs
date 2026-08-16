//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 641/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk641(t129: f64, t8153: f64, t135: f64, t138: f64, t8157: f64, t120: f64, t1655: f64, t40: f64, t6: f64, t12: f64, t171: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8935 = t129 * t8153;
    let t8937 = t8157 * t135 * t138;
    let t8942 = t120 * t1655;
    let t8946 = t6 / t40;
    let t8947 = t12 * t171;
    let t8948 = t8946 * t8947;
    (t8935, t8937, t8942, t8946, t8947, t8948)
}
