//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2044/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2044(t5883: f64, t7356: f64, t108710: f64, t109153: f64, t109242: f64, t13426: f64, t18227: f64, t2055: f64, t2322: f64, t27123: f64, t28219: f64, t28683: f64, t30138: f64, t30143: f64, t30570: f64, t4248: f64, t5523: f64, t7373: f64, t7889: f64, t7983: f64) -> (f64, f64) {
    let t111066 = t7356 * t5883;
    let t111068 = 2.0_f64 * t108710 * t2055 + 4.0_f64 * t109153 * t2055 + 2.0_f64 * t109242 * t2055 + 4.0_f64 * t13426 * t7983 + 4.0_f64 * t18227 * t7983 + 2.0_f64 * t2322 * t30570 + 4.0_f64 * t27123 * t7983 + 4.0_f64 * t28219 * t7983 + 4.0_f64 * t28683 * t4248 + 4.0_f64 * t28683 * t7889 + 4.0_f64 * t30138 * t7373 + 2.0_f64 * t30143 * t7373 + 2.0_f64 * t30570 * t5523 + 2.0_f64 * t111066;
    (t111066, t111068)
}
