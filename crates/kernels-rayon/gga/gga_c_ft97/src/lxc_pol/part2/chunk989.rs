//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 989/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk989(t1882: f64, t4276: f64, t4280: f64, t10443: f64, t4146: f64, t10533: f64, t10539: f64, t10545: f64, t10670: f64, t10678: f64, t10693: f64, t15309: f64, t15314: f64, t15318: f64, t15322: f64, t15325: f64, t15329: f64, t1901: f64, t446: f64) -> f64 {
    let t15334 = 2.0_f64 / 9.0_f64 * t1882 * t4276;
    let t15336 = 2.0_f64 / 9.0_f64 * t1882 * t4280;
    let t15338 = t10443 * t4146;
    let t15341 = -2.0_f64 / 9.0_f64 * t1901 * t15309 - 4.0_f64 / 9.0_f64 * t1901 * t15314 - 2.0_f64 / 9.0_f64 * t10533 - 4.0_f64 / 81.0_f64 * t15318 + 2.0_f64 / 27.0_f64 * t10539 - 2.0_f64 / 27.0_f64 * t10545 - 2.0_f64 / 3.0_f64 * t446 * t15322 - 2.0_f64 / 3.0_f64 * t446 * t15325 + 4.0_f64 / 27.0_f64 * t15329 + 2.0_f64 / 81.0_f64 * t10670 + t10678 / 27.0_f64 + t15334 + t15336 + 2.0_f64 / 9.0_f64 * t10693 + 2.0_f64 / 9.0_f64 * t1901 * t15338;
    t15341
}
