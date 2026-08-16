//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 853/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk853(t1058: f64, t7126: f64, t1973: f64, t3201: f64, t7114: f64, t1020: f64, t7131: f64, t1971: f64, t3104: f64, t351: f64, t25516: f64, t3114: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t25557 = t7126 * t1058;
    let t25560 = 0.95275595817932748827e-4_f64 * t1973 * t3201;
    let t25564 = t7114 * t1058;
    let t25569 = t1020 * t7131;
    let t25576 = t1971 * t3104;
    let t25577 = t351 * t25576;
    let t25580 = t3114 * t25516;
    (t25557, t25560, t25564, t25569, t25577, t25580)
}
