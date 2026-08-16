//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3253/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3253(t10298: f64, t10309: f64, t10310: f64, t10410: f64, t13269: f64, t13283: f64, t13420: f64, t1497: f64, t2242: f64, t2247: f64, t2315: f64, t4173: f64, t4178: f64, t4241: f64, t45955: f64, t45963: f64, t45972: f64, t60248: f64, t603: f64, t60360: f64, t60391: f64, t60417: f64, t60483: f64, t644: f64) -> f64 {
    let t60496 = 20.0_f64 * t2247 * t1497 * t10410 - 12.0_f64 * t60248 * t644 - 12.0_f64 * t13269 * t2315 - 4.0_f64 * t4173 * t10410 - 4.0_f64 * t45955 * t1497 - 12.0_f64 * t10298 * t4241 - 12.0_f64 * t2242 * t13420 - 4.0_f64 * t603 * (t60360 + t60391 + t60417 + t60483) - 360.0_f64 * t45963 * t13283 + 840.0_f64 * t45972 * t1497 * t10310 - 360.0_f64 * t10309 * t4178 * t2315;
    t60496
}
