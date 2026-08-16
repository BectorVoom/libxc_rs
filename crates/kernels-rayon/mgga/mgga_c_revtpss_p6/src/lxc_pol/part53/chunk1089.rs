//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1089/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1089(t125: f64, t2769: f64, t1032: f64, t1949: f64, t867: f64, t786: f64, t25296: f64, t243: f64, t257: f64, t9794: f64, t25304: f64, t8464: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t119852 = t125 * t2769;
    let t119857 = t1949 * t1032;
    let t119858 = t119857 * t867;
    let t119859 = t786 * t119858;
    let t119860 = t119859 * t25296;
    let t119867 = t243 * t257;
    let t119868 = t9794 * t119867;
    let t119869 = t25304 * t8464 * t119868;
    (t119852, t119857, t119858, t119859, t119860, t119867, t119868, t119869)
}
