//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1067/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1067(t21460: f64, t739: f64, t5654: f64, t7802: f64, t10912: f64, t1422: f64, t787: f64, t2672: f64, t6081: f64, t1980: f64, t7339: f64, t20157: f64, t805: f64, t831: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t23433 = t739 * t21460;
    let t23469 = t5654 * t7802;
    let t23477 = t787 * t10912 * t1422;
    let t23492 = t6081 * t2672;
    let t23495 = t1980 * t7339;
    let t23516 = t805 * t831 * t20157;
    (t23433, t23469, t23477, t23492, t23495, t23516)
}
