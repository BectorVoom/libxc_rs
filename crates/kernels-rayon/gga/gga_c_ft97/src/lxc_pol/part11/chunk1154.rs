//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1154/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1154(t10690: f64, t1882: f64, t2832: f64, t848: f64, t10810: f64, t1901: f64, t2682: f64, t2862: f64, t2884: f64, t2894: f64, t296: f64, t319: f64, t43525: f64, t44272: f64, t44276: f64, t44278: f64, t44280: f64, t44289: f64, t44292: f64, t44294: f64, t446: f64, t684: f64, t835: f64) -> f64 {
    let t44300 = t1882 * t10690;
    let t44302 = t848 * t2832;
    let t44306 = -2.0_f64 * t446 * t296 * t44272 + 4.0_f64 / 3.0_f64 * t44276 + 4.0_f64 / 9.0_f64 * t44278 + 8.0_f64 * t446 * t44280 * t319 * t43525 - 4.0_f64 / 9.0_f64 * t446 * t835 * t10810 * t684 - 8.0_f64 / 3.0_f64 * t44289 + 8.0_f64 / 9.0_f64 * t44292 - 16.0_f64 / 9.0_f64 * t44294 + 4.0_f64 * t446 * t2862 * t2894 * t2682 - 8.0_f64 / 3.0_f64 * t44300 + 4.0_f64 / 3.0_f64 * t1901 * t44302 * t2884;
    t44306
}
