//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 505/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk505(t1851: f64, t3: f64, t1401: f64, t1458: f64, t577: f64, t193: f64, t202: f64, t154: f64, t204: f64, t119: f64, t210: f64, t201: f64, t243: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1852 = t3 * t1851;
    let t1858 = 0.45e1_f64 * t1851 * t577 + 0.135e2_f64 * t1401 * t1458;
    let t1877 = t193 * t202;
    let t1878 = t204 * t154;
    let t1887 = t210 * t119;
    let t1891 = 1.0_f64 / t243 / t201;
    (t1852, t1858, t1877, t1878, t1887, t1891)
}
