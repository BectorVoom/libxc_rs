//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 840/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk840(t37334: f64, t1882: f64, t7812: f64, t28: f64, t7755: f64, t8183: f64, t89: f64, t1581: f64, t7773: f64, t1554: f64, t1636: f64, t1560: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t37335 = 8.0_f64 / 27.0_f64 * t37334;
    let t37336 = t1882 * t7812;
    let t37340 = t89 * t28 * t7755 * t8183;
    let t37343 = t89 * t7773 * t1581;
    let t37344 = 4.0_f64 / 27.0_f64 * t37343;
    let t37345 = t1636 * t1554;
    let t37347 = t89 * t37345 * t1560;
    (t37335, t37336, t37340, t37343, t37344, t37345, t37347)
}
