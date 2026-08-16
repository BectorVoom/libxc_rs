//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 630/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk630(t14: f64, t15564: f64, t15565: f64, t81: f64, t8633: f64, t2258: f64, t342: f64, t4410: f64, t630: f64, t4436: f64, t7241: f64, t4418: f64, t7780: f64, t89: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15567 = t15564 * t15565 * t14;
    let t15568 = t8633 * t81;
    let t15575 = t2258 * t81;
    let t15584 = t342 * t630 * t4410;
    let t15601 = t7241 * t4436;
    let t15606 = t89 * t7780 * t4418;
    (t15567, t15568, t15575, t15584, t15601, t15606)
}
