//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1182/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1182(t299: f64, t36276: f64, t5: f64, t113: f64, t1275: f64, t144289: f64, t152530: f64, t152574: f64, t152615: f64, t152644: f64, t153492: f64, t153520: f64, t153548: f64, t153674: f64, t153712: f64, t154908: f64, t154945: f64, t154983: f64, t155009: f64, t155028: f64, t155066: f64, t155092: f64, t332: f64, t34338: f64, t34341: f64, t36277: f64, t4377: f64, t4382: f64, t4385: f64, t4391: f64, t4395: f64, t505: f64, t911: f64, t992: f64) -> f64 {
    let t300 = 10000000.0_f64 <= t299;
    let t155101 = t5 * t36276;
    let t155123 = piecewise3(t300, 0.0_f64, t5 * (t152530 + t152574 + t152615 + t152644 + t153492 + t153520 + t153548 + t153674 + t153712 + t154908 + t154945 + t154983 + t155009 + t155028 + t155066 + t155092) * t332 * t113 / 4.0_f64 + t155101 * t911 / 4.0_f64 + t5 * t36277 * t505 / 4.0_f64 + t144289 * t1275 / 4.0_f64 + t34341 * t4377 / 4.0_f64 + t34341 * t4382 / 4.0_f64 + t34341 * t4385 / 4.0_f64 + t5 * t34338 * t992 / 4.0_f64 + t34341 * t4391 / 4.0_f64 - t34341 * t4395 / 2.0_f64);
    t155123
}
