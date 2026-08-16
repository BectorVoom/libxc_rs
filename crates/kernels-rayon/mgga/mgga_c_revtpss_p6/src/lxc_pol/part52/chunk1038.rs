//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1038/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1038(t6993: f64, t7359: f64, t3140: f64, t860: f64, t8477: f64, t1949: f64, t7398: f64, t8650: f64, t1032: f64, t2061: f64, t1955: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t32421 = 2.0_f64 * t7359 * t6993;
    let t32425 = t860 * t3140;
    let t32426 = t8477 * t32425;
    let t32429 = t7398 * t1949;
    let t32430 = t8650 * t32429;
    let t32433 = t2061 * t1032;
    let t32434 = t1955 * t32433;
    (t32421, t32425, t32426, t32429, t32430, t32433, t32434)
}
