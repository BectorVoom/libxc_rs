//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 778/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk778(t2405: f64, t2857: f64, t882: f64, t319: f64, t835: f64, t9596: f64, t1882: f64, t2864: f64, t2894: f64, t684: f64, t2850: f64, t9587: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10526 = t2857 * t882 * t2405;
    let t10530 = t835 * t319 * t9596;
    let t10533 = t1882 * t2864;
    let t10536 = t835 * t2894 * t684;
    let t10539 = t1882 * t2850;
    let t10542 = t835 * t319 * t9587;
    (t10526, t10530, t10533, t10536, t10539, t10542)
}
