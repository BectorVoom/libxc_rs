//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1142/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1142(t1450: f64, t6836: f64, t25864: f64, t2014: f64, t1843: f64, t7741: f64, t651: f64, t196: f64, t197: f64, t6773: f64, t2035: f64, t5920: f64, t94: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t29498 = t1450 * t6836;
    let t29499 = t25864 * t29498;
    let t29501 = 6.0_f64 * t2014 * t29499;
    let t29502 = t1843 * t7741;
    let t29504 = 4.0_f64 * t651 * t29502;
    let t29506 = t6773 * t196 * t197;
    let t29507 = t29506 * t2035;
    let t29508 = t94 * t5920;
    (t29498, t29499, t29501, t29502, t29504, t29506, t29507, t29508)
}
