//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1038/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1038(t31773: f64, t8916: f64, t7447: f64, t8920: f64, t1439: f64, t1983: f64, t7380: f64, t1460: f64, t1992: f64, t2095: f64, t30225: f64, t532: f64) -> (f64, f64, f64, f64, f64) {
    let t36353 = t31773 * t8916;
    let t36355 = t7447 * t8920;
    let t36364 = t7380 * t1983 * t1439;
    let t36367 = t2095 * t1992 * t1460;
    let t36370 = t30225 * t532;
    (t36353, t36355, t36364, t36367, t36370)
}
