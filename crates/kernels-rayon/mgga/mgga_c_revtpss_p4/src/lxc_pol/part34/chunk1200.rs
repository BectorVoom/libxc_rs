//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1200/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1200(t11874: f64, t27492: f64, t11970: f64, t1973: f64, t11858: f64, t11926: f64, t25516: f64, t11940: f64, t1972: f64, t11735: f64, t1968: f64, t11772: f64, t25515: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t93548 = t11874 * t27492;
    let t93611 = 0.1270341277572436651e-3_f64 * t1973 * t11970;
    let t93658 = t11858 * t27492;
    let t93667 = t11926 * t25516;
    let t93725 = t11940 * t1972;
    let t93750 = 5.0_f64 / 1296.0_f64 * t1968 * t11735;
    let t93751 = t25515 * t11772;
    (t93548, t93611, t93658, t93667, t93725, t93750, t93751)
}
