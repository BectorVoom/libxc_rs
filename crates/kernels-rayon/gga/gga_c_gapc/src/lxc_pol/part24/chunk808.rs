//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 808/1327 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk808(t3384: f64, t9661: f64, t787: f64, t7927: f64, t3396: f64, t325: f64, t8992: f64, t2817: f64, t3321: f64, t3320: f64, t1084: f64, t8686: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9662 = t9661 * t3384;
    let t9664 = t7927 * t787;
    let t9665 = t3396 * t9664;
    let t9667 = t325 * t8992;
    let t9668 = t9667 * t2817;
    let t9670 = t7927 * t3321;
    let t9671 = t3320 * t9670;
    let t9673 = t1084 * t8686;
    (t9662, t9665, t9668, t9670, t9671, t9673)
}
