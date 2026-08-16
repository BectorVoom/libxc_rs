//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1120/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1120(t119868: f64, t25304: f64, t8464: f64, t233: f64, t240: f64, t31752: f64, t843: f64, t31774: f64, t31769: f64, t124: f64, t867: f64, t14686: f64, t886: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t119869 = t25304 * t8464 * t119868;
    let t119875 = t31752 * t233 * t843 * t240;
    let t119876 = t119875 * t31774;
    let t119877 = 0.263521689745817692e-2_f64 * t119876;
    let t119878 = t119875 * t31769;
    let t119879 = 0.1054086758983270768e-1_f64 * t119878;
    let t119891 = t124 * t867;
    let t119893 = t14686 * t119891 * t886;
    (t119869, t119875, t119877, t119879, t119891, t119893)
}
