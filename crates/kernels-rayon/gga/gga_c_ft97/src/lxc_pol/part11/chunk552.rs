//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 552/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk552(t7800: f64, t82: f64, t7765: f64, t1555: f64, t89: f64, t1557: f64, t363: f64, t1580: f64) -> (f64, f64, f64, f64, f64) {
    let t7801 = t82 * t7800;
    let t7802 = t7801 * t7765;
    let t7804 = t89 * t1555 * t7802;
    let t7806 = t1557 * t363;
    let t7807 = t7806 * t1580;
    (t7801, t7802, t7804, t7806, t7807)
}
