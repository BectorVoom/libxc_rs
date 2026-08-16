//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 991/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk991(t33068: f64, t8392: f64, t32992: f64, t604: f64, t33036: f64, t2101: f64, t7390: f64, t7312: f64, t358: f64, t7407: f64, t33204: f64, t33200: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t139634 = t8392 * t33068;
    let t139661 = t604 * t32992;
    let t139666 = t8392 * t33036;
    let t139675 = t2101 * t7390;
    let t139679 = t604 * t7312;
    let t139702 = t7407 * t358;
    let t139716 = t8392 * t33204;
    let t139722 = t8392 * t33200;
    (t139634, t139661, t139666, t139675, t139679, t139702, t139716, t139722)
}
