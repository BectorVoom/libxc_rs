//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1165/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1165(t294: f64, t9570: f64, t2252: f64, t2644: f64, t342: f64, t10231: f64, t630: f64, t784: f64, t8639: f64, t10236: f64, t10388: f64, t10410: f64, t10422: f64, t10426: f64, t10432: f64, t13605: f64, t1526: f64, t231: f64, t2320: f64, t2639: f64, t343: f64, t3806: f64, t8608: f64, t9571: f64) -> f64 {
    let t44700 = t294 * t9570;
    let t44709 = t342 * t2252 * t2644;
    let t44712 = t342 * t630 * t10231;
    let t44716 = 5.0_f64 / 54.0_f64 * t342 * t8639 * t784;
    let t44717 = t10236 - t342 * t343 * t231 * t10388 / 4.0_f64 - t1526 * t2320 * t10422 / 4.0_f64 - t1526 * t2320 * t2639 * t8608 / 12.0_f64 - t1526 * t3806 * t10410 / 3.0_f64 - 7.0_f64 / 27.0_f64 * t1526 * t13605 * t44700 * t9571 - t1526 * t2320 * t10426 / 4.0_f64 + t10432 + t44709 / 6.0_f64 - t44712 / 4.0_f64 - t44716;
    t44717
}
