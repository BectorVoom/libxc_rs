//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1105/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1105(t1968: f64, t3080: f64, t1973: f64, t3201: f64, t25516: f64, t3114: f64, t3057: f64, t7143: f64, t1035: f64, t8515: f64, t1983: f64, t378: f64, t7150: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t25538 = t1968 * t3080 / 432.0_f64;
    let t25560 = 0.95275595817932748827e-4_f64 * t1973 * t3201;
    let t25580 = t3114 * t25516;
    let t25591 = t3057 * t7143;
    let t25604 = t8515 * t1035;
    let t25605 = t1983 * t25604;
    let t25610 = t7150 * t378;
    (t25538, t25560, t25580, t25591, t25604, t25605, t25610)
}
