//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 873/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk873(t836: f64, t8720: f64, t568: f64, t1880: f64, t2958: f64, t1445: f64, t2949: f64, t2950: f64, t4614: f64, t1457: f64, t1035: f64, t2052: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8721 = t836 * t8720;
    let t8722 = t568 * t8721;
    let t8725 = t2958 * t1880;
    let t8726 = t1445 * t8725;
    let t8729 = t2949 * t1880;
    let t8730 = t1445 * t8729;
    let t8733 = t4614 * t2950;
    let t8738 = t1457 * t8729;
    let t8741 = t2052 * t1035;
    (t8722, t8726, t8729, t8730, t8733, t8738, t8741)
}
