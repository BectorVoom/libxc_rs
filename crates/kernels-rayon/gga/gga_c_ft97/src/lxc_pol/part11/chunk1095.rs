//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1095/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1095(t2614: f64, t8232: f64, t10137: f64, t1882: f64, t8392: f64, t9855: f64, t10155: f64, t10079: f64, t10121: f64, t1901: f64, t1934: f64, t2373: f64, t2594: f64, t2600: f64, t2606: f64, t265: f64, t41691: f64, t41718: f64, t41726: f64, t42884: f64, t42894: f64, t446: f64, t684: f64, t724: f64, t761: f64) -> f64 {
    let t42896 = t8232 * t2614;
    let t42898 = t1882 * t10137;
    let t42914 = t8392 * t9855;
    let t42916 = t1882 * t10155;
    let t42918 = -8.0_f64 / 27.0_f64 * t42884 + 8.0_f64 / 3.0_f64 * t446 * t724 * t265 * t41691 - 8.0_f64 / 3.0_f64 * t446 * t2594 * t265 * t41718 + 112.0_f64 / 243.0_f64 * t42894 + 16.0_f64 / 27.0_f64 * t42896 + 4.0_f64 / 9.0_f64 * t42898 + 4.0_f64 / 9.0_f64 * t1901 * t2606 * t761 * t10121 * t684 - 2.0_f64 / 9.0_f64 * t446 * t2594 * t265 * t41726 - 4.0_f64 / 3.0_f64 * t1901 * t10079 * t2600 * t1934 * t2373 - 8.0_f64 / 9.0_f64 * t42914 + 4.0_f64 / 3.0_f64 * t42916;
    t42918
}
