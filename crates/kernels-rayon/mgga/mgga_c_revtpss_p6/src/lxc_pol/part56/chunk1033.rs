//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1033/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1033(t119868: f64, t25304: f64, t8464: f64, t233: f64, t240: f64, t31752: f64, t843: f64, t31774: f64, t31769: f64, t41077: f64, t822: f64, t119858: f64, t7063: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t119869 = t25304 * t8464 * t119868;
    let t119870 = 0.17851433602423232928e-4_f64 * t119869;
    let t119875 = t31752 * t233 * t843 * t240;
    let t119876 = t119875 * t31774;
    let t119878 = t119875 * t31769;
    let t119883 = t41077 * t822;
    let t119888 = t7063 * t119858;
    (t119870, t119875, t119876, t119878, t119883, t119888)
}
