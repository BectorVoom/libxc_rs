//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1126/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1126(t243: f64, t257: f64, t9794: f64, t25304: f64, t8464: f64, t233: f64, t240: f64, t31752: f64, t843: f64, t31774: f64, t31769: f64, t124: f64, t867: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t119867 = t243 * t257;
    let t119868 = t9794 * t119867;
    let t119869 = t25304 * t8464 * t119868;
    let t119875 = t31752 * t233 * t843 * t240;
    let t119876 = t119875 * t31774;
    let t119877 = 0.263521689745817692e-2_f64 * t119876;
    let t119878 = t119875 * t31769;
    let t119879 = 0.1054086758983270768e-1_f64 * t119878;
    let t119891 = t124 * t867;
    (t119867, t119868, t119869, t119875, t119877, t119879, t119891)
}
