//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 589/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk589(t14: f64, t6817: f64, t231: f64, t213: f64, t679: f64, t689: f64, t6979: f64, t709: f64, t3817: f64, t6027: f64, t2441: f64, t3886: f64) -> (f64, f64, f64, f64, f64) {
    let t27617 = t6817 * t14;
    let t27618 = t27617 * t231;
    let t27619 = t213 * t679;
    let t27620 = t27619 * t689;
    let t27621 = t27618 * t27620;
    let t27625 = t6979 * t709;
    let t27629 = t6027 * t3817;
    let t27633 = t2441 * t3886;
    (t27617, t27621, t27625, t27629, t27633)
}
